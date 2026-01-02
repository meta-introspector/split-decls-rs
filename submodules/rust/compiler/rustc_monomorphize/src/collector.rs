mkmod!{autodiff, { 
                getname!(autodiff);
                getsrc!(autodiff);
                getpath!(autodiff);
                get_deps!(autodiff);
                get_crates!(autodiff);
                mkinclude!(autodiff);
                 
            }}
mkuse!{use std :: cell :: OnceCell ;}
mkuse!{use rustc_data_structures :: fx :: FxIndexMap ;}
mkuse!{use rustc_data_structures :: sync :: { MTLock , par_for_each_in } ;}
mkuse!{use rustc_data_structures :: unord :: { UnordMap , UnordSet } ;}
mkuse!{use rustc_hir as hir ;}
mkuse!{use rustc_hir :: attrs :: InlineAttr ;}
mkuse!{use rustc_hir :: def :: DefKind ;}
mkuse!{use rustc_hir :: def_id :: { DefId , DefIdMap , LocalDefId } ;}
mkuse!{use rustc_hir :: lang_items :: LangItem ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: mir :: interpret :: { AllocId , ErrorHandled , GlobalAlloc , Scalar } ;}
mkuse!{use rustc_middle :: mir :: mono :: { CollectionMode , InstantiationMode , MonoItem } ;}
mkuse!{use rustc_middle :: mir :: visit :: Visitor as MirVisitor ;}
mkuse!{use rustc_middle :: mir :: { self , Location , MentionedItem , traversal } ;}
mkuse!{use rustc_middle :: query :: TyCtxtAt ;}
mkuse!{use rustc_middle :: ty :: adjustment :: { CustomCoerceUnsized , PointerCoercion } ;}
mkuse!{use rustc_middle :: ty :: layout :: ValidityRequirement ;}
mkuse!{use rustc_middle :: ty :: { self , GenericArgs , GenericParamDefKind , Instance , InstanceKind , Ty , TyCtxt , TypeFoldable , TypeVisitableExt , VtblEntry , } ;}
mkuse!{use rustc_middle :: util :: Providers ;}
mkuse!{use rustc_middle :: { bug , span_bug } ;}
mkuse!{use rustc_session :: config :: { DebugInfo , EntryFnType } ;}
mkuse!{use rustc_span :: source_map :: { Spanned , dummy_spanned , respan } ;}
mkuse!{use rustc_span :: { DUMMY_SP , Span } ;}
mkuse!{use tracing :: { debug , instrument , trace } ;}
mkuse!{use crate :: collector :: autodiff :: collect_autodiff_fn ;}
mkuse!{use crate :: errors :: { self , EncounteredErrorWhileInstantiating , EncounteredErrorWhileInstantiatingGlobalAsm , NoOptimizedMir , RecursionLimit , } ;}
mkitem!{mkenum!{# [derive (PartialEq)] pub (crate) enum MonoItemCollectionStrategy { Eager , Lazy , }}}
mkitem!{mkstruct!{# [doc = " The state that is shared across the concurrent threads that are doing collection."] struct SharedState < 'tcx > { # [doc = " Items that have been or are currently being recursively collected."] visited : MTLock < UnordSet < MonoItem < 'tcx > > > , # [doc = " Items that have been or are currently being recursively treated as \"mentioned\", i.e., their"] # [doc = " consts are evaluated but nothing is added to the collection."] mentioned : MTLock < UnordSet < MonoItem < 'tcx > > > , # [doc = " Which items are being used where, for better errors."] usage_map : MTLock < UsageMap < 'tcx > > , }}}
mkitem!{mkstruct!{pub (crate) struct UsageMap < 'tcx > { pub used_map : UnordMap < MonoItem < 'tcx > , Vec < MonoItem < 'tcx > > > , user_map : UnordMap < MonoItem < 'tcx > , Vec < MonoItem < 'tcx > > > , }}}
mkitem!{mkimpl!{impl < 'tcx > UsageMap < 'tcx > { fn new () -> UsageMap < 'tcx > { UsageMap { used_map : Default :: default () , user_map : Default :: default () } } fn record_used < 'a > (& mut self , user_item : MonoItem < 'tcx > , used_items : & 'a MonoItems < 'tcx >) where 'tcx : 'a , { for used_item in used_items . items () { self . user_map . entry (used_item) . or_default () . push (user_item) ; } assert ! (self . used_map . insert (user_item , used_items . items () . collect ()) . is_none ()) ; } pub (crate) fn get_user_items (& self , item : MonoItem < 'tcx >) -> & [MonoItem < 'tcx >] { self . user_map . get (& item) . map (| items | items . as_slice ()) . unwrap_or (& []) } # [doc = " Internally iterate over all inlined items used by `item`."] pub (crate) fn for_each_inlined_used_item < F > (& self , tcx : TyCtxt < 'tcx > , item : MonoItem < 'tcx > , mut f : F ,) where F : FnMut (MonoItem < 'tcx >) , { let used_items = self . used_map . get (& item) . unwrap () ; for used_item in used_items . iter () { let is_inlined = used_item . instantiation_mode (tcx) == InstantiationMode :: LocalCopy ; if is_inlined { f (* used_item) ; } } } }}}
mkitem!{mkstruct!{struct MonoItems < 'tcx > { items : FxIndexMap < MonoItem < 'tcx > , Span > , }}}
mkitem!{mkimpl!{impl < 'tcx > MonoItems < 'tcx > { fn new () -> Self { Self { items : FxIndexMap :: default () } } fn is_empty (& self) -> bool { self . items . is_empty () } fn push (& mut self , item : Spanned < MonoItem < 'tcx > >) { self . items . entry (item . node) . or_insert (item . span) ; } fn items (& self) -> impl Iterator < Item = MonoItem < 'tcx > > { self . items . keys () . cloned () } }}}
mkitem!{mkimpl!{impl < 'tcx > IntoIterator for MonoItems < 'tcx > { type Item = Spanned < MonoItem < 'tcx > > ; type IntoIter = impl Iterator < Item = Spanned < MonoItem < 'tcx > > > ; fn into_iter (self) -> Self :: IntoIter { self . items . into_iter () . map (| (item , span) | respan (span , item)) } }}}
mkitem!{mkimpl!{impl < 'tcx > Extend < Spanned < MonoItem < 'tcx > > > for MonoItems < 'tcx > { fn extend < I > (& mut self , iter : I) where I : IntoIterator < Item = Spanned < MonoItem < 'tcx > > > , { for item in iter { self . push (item) } } }}}

macro_rules! collect_items_root_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_items_root in module {}", module_path!());
    };
}

mkfn!{
    collect_items_root_introspect!();
    fn collect_items_root < 'tcx > (tcx : TyCtxt < 'tcx > , starting_item : Spanned < MonoItem < 'tcx > > , state : & SharedState < 'tcx > , recursion_limit : Limit ,) { if ! state . visited . lock_mut () . insert (starting_item . node) { return ; } let mut recursion_depths = DefIdMap :: default () ; collect_items_rec (tcx , starting_item , state , & mut recursion_depths , recursion_limit , CollectionMode :: UsedItems ,) ; }
}

macro_rules! collect_items_rec_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_items_rec in module {}", module_path!());
    };
}

mkfn!{
    collect_items_rec_introspect!();
    # [doc = " Collect all monomorphized items reachable from `starting_point`, and emit a note diagnostic if a"] # [doc = " post-monomorphization error is encountered during a collection step."] # [doc = ""] # [doc = " `mode` determined whether we are scanning for [used items][CollectionMode::UsedItems]"] # [doc = " or [mentioned items][CollectionMode::MentionedItems]."] # [instrument (skip (tcx , state , recursion_depths , recursion_limit) , level = "debug")] fn collect_items_rec < 'tcx > (tcx : TyCtxt < 'tcx > , starting_item : Spanned < MonoItem < 'tcx > > , state : & SharedState < 'tcx > , recursion_depths : & mut DefIdMap < usize > , recursion_limit : Limit , mode : CollectionMode ,) { let mut used_items = MonoItems :: new () ; let mut mentioned_items = MonoItems :: new () ; let recursion_depth_reset ; let error_count = tcx . dcx () . err_count () ; match starting_item . node { MonoItem :: Static (def_id) => { recursion_depth_reset = None ; if mode == CollectionMode :: UsedItems { let instance = Instance :: mono (tcx , def_id) ; debug_assert ! (tcx . should_codegen_locally (instance)) ; let DefKind :: Static { nested , .. } = tcx . def_kind (def_id) else { bug ! () } ; if ! nested { let ty = instance . ty (tcx , ty :: TypingEnv :: fully_monomorphized ()) ; visit_drop_use (tcx , ty , true , starting_item . span , & mut used_items) ; } if let Ok (alloc) = tcx . eval_static_initializer (def_id) { for & prov in alloc . inner () . provenance () . ptrs () . values () { collect_alloc (tcx , prov . alloc_id () , & mut used_items) ; } } if tcx . needs_thread_local_shim (def_id) { used_items . push (respan (starting_item . span , MonoItem :: Fn (Instance { def : InstanceKind :: ThreadLocalShim (def_id) , args : GenericArgs :: empty () , }) ,)) ; } } } MonoItem :: Fn (instance) => { debug_assert ! (tcx . should_codegen_locally (instance)) ; recursion_depth_reset = Some (check_recursion_limit (tcx , instance , starting_item . span , recursion_depths , recursion_limit ,)) ; rustc_data_structures :: stack :: ensure_sufficient_stack (| | { let (used , mentioned) = tcx . items_of_instance ((instance , mode)) ; used_items . extend (used . into_iter () . copied ()) ; mentioned_items . extend (mentioned . into_iter () . copied ()) ; }) ; } MonoItem :: GlobalAsm (item_id) => { assert ! (mode == CollectionMode :: UsedItems , "should never encounter global_asm when collecting mentioned items") ; recursion_depth_reset = None ; let item = tcx . hir_item (item_id) ; if let hir :: ItemKind :: GlobalAsm { asm , .. } = item . kind { for (op , op_sp) in asm . operands { match * op { hir :: InlineAsmOperand :: Const { .. } => { } hir :: InlineAsmOperand :: SymFn { expr } => { let fn_ty = tcx . typeck (item_id . owner_id) . expr_ty (expr) ; visit_fn_use (tcx , fn_ty , false , * op_sp , & mut used_items) ; } hir :: InlineAsmOperand :: SymStatic { path : _ , def_id } => { let instance = Instance :: mono (tcx , def_id) ; if tcx . should_codegen_locally (instance) { trace ! ("collecting static {:?}" , def_id) ; used_items . push (dummy_spanned (MonoItem :: Static (def_id))) ; } } hir :: InlineAsmOperand :: In { .. } | hir :: InlineAsmOperand :: Out { .. } | hir :: InlineAsmOperand :: InOut { .. } | hir :: InlineAsmOperand :: SplitInOut { .. } | hir :: InlineAsmOperand :: Label { .. } => { span_bug ! (* op_sp , "invalid operand type for global_asm!") } } } } else { span_bug ! (item . span , "Mismatch between hir::Item type and MonoItem type") } } } ; if tcx . dcx () . err_count () > error_count && starting_item . node . is_generic_fn () && starting_item . node . is_user_defined () { match starting_item . node { MonoItem :: Fn (instance) => tcx . dcx () . emit_note (EncounteredErrorWhileInstantiating { span : starting_item . span , kind : "fn" , instance , }) , MonoItem :: Static (def_id) => tcx . dcx () . emit_note (EncounteredErrorWhileInstantiating { span : starting_item . span , kind : "static" , instance : Instance :: new_raw (def_id , GenericArgs :: empty ()) , }) , MonoItem :: GlobalAsm (_) => { tcx . dcx () . emit_note (EncounteredErrorWhileInstantiatingGlobalAsm { span : starting_item . span , }) } } } if mode == CollectionMode :: UsedItems { state . usage_map . lock_mut () . record_used (starting_item . node , & used_items) ; } { let mut visited = OnceCell :: default () ; if mode == CollectionMode :: UsedItems { used_items . items . retain (| k , _ | visited . get_mut_or_init (| | state . visited . lock_mut ()) . insert (* k)) ; } let mut mentioned = OnceCell :: default () ; mentioned_items . items . retain (| k , _ | { ! visited . get_or_init (| | state . visited . lock ()) . contains (k) && mentioned . get_mut_or_init (| | state . mentioned . lock_mut ()) . insert (* k) }) ; } if mode == CollectionMode :: MentionedItems { assert ! (used_items . is_empty () , "'mentioned' collection should never encounter used items") ; } else { for used_item in used_items { collect_items_rec (tcx , used_item , state , recursion_depths , recursion_limit , CollectionMode :: UsedItems ,) ; } } for mentioned_item in mentioned_items { collect_items_rec (tcx , mentioned_item , state , recursion_depths , recursion_limit , CollectionMode :: MentionedItems ,) ; } if let Some ((def_id , depth)) = recursion_depth_reset { recursion_depths . insert (def_id , depth) ; } }
}

macro_rules! check_recursion_limit_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function check_recursion_limit in module {}", module_path!());
    };
}

mkfn!{
    check_recursion_limit_introspect!();
    fn check_recursion_limit < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , span : Span , recursion_depths : & mut DefIdMap < usize > , recursion_limit : Limit ,) -> (DefId , usize) { let def_id = instance . def_id () ; let recursion_depth = recursion_depths . get (& def_id) . cloned () . unwrap_or (0) ; debug ! (" => recursion depth={}" , recursion_depth) ; let adjusted_recursion_depth = if tcx . is_lang_item (def_id , LangItem :: DropInPlace) { recursion_depth / 4 } else { recursion_depth } ; if ! recursion_limit . value_within_limit (adjusted_recursion_depth) { let def_span = tcx . def_span (def_id) ; let def_path_str = tcx . def_path_str (def_id) ; tcx . dcx () . emit_fatal (RecursionLimit { span , instance , def_span , def_path_str }) ; } recursion_depths . insert (def_id , recursion_depth + 1) ; (def_id , recursion_depth) }
}
mkitem!{mkstruct!{struct MirUsedCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , body : & 'a mir :: Body < 'tcx > , used_items : & 'a mut MonoItems < 'tcx > , # [doc = " See the comment in `collect_items_of_instance` for the purpose of this set."] # [doc = " Note that this contains *not-monomorphized* items!"] used_mentioned_items : & 'a mut UnordSet < MentionedItem < 'tcx > > , instance : Instance < 'tcx > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > MirUsedCollector < 'a , 'tcx > { fn monomorphize < T > (& self , value : T) -> T where T : TypeFoldable < TyCtxt < 'tcx > > , { trace ! ("monomorphize: self.instance={:?}" , self . instance) ; self . instance . instantiate_mir_and_normalize_erasing_regions (self . tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (value) ,) } # [doc = " Evaluates a *not yet monomorphized* constant."] fn eval_constant (& mut self , constant : & mir :: ConstOperand < 'tcx >) -> Option < mir :: ConstValue > { let const_ = self . monomorphize (constant . const_) ; match const_ . eval (self . tcx , ty :: TypingEnv :: fully_monomorphized () , constant . span) { Ok (v) => Some (v) , Err (ErrorHandled :: TooGeneric (..)) => span_bug ! (constant . span , "collection encountered polymorphic constant: {:?}" , const_) , Err (err @ ErrorHandled :: Reported (..)) => { err . emit_note (self . tcx) ; return None ; } } } }}}
mkitem!{mkimpl!{impl < 'a , 'tcx > MirVisitor < 'tcx > for MirUsedCollector < 'a , 'tcx > { fn visit_rvalue (& mut self , rvalue : & mir :: Rvalue < 'tcx > , location : Location) { debug ! ("visiting rvalue {:?}" , * rvalue) ; let span = self . body . source_info (location) . span ; match * rvalue { mir :: Rvalue :: Cast (mir :: CastKind :: PointerCoercion (PointerCoercion :: Unsize , _) , ref operand , target_ty ,) => { let source_ty = operand . ty (self . body , self . tcx) ; self . used_mentioned_items . insert (MentionedItem :: UnsizeCast { source_ty , target_ty }) ; let target_ty = self . monomorphize (target_ty) ; let source_ty = self . monomorphize (source_ty) ; let (source_ty , target_ty) = find_tails_for_unsizing (self . tcx . at (span) , source_ty , target_ty) ; if target_ty . is_trait () && ! source_ty . is_trait () { create_mono_items_for_vtable_methods (self . tcx , target_ty , source_ty , span , self . used_items ,) ; } } mir :: Rvalue :: Cast (mir :: CastKind :: PointerCoercion (PointerCoercion :: ReifyFnPointer , _) , ref operand , _ ,) => { let fn_ty = operand . ty (self . body , self . tcx) ; self . used_mentioned_items . insert (MentionedItem :: Fn (fn_ty)) ; let fn_ty = self . monomorphize (fn_ty) ; visit_fn_use (self . tcx , fn_ty , false , span , self . used_items) ; } mir :: Rvalue :: Cast (mir :: CastKind :: PointerCoercion (PointerCoercion :: ClosureFnPointer (_) , _) , ref operand , _ ,) => { let source_ty = operand . ty (self . body , self . tcx) ; self . used_mentioned_items . insert (MentionedItem :: Closure (source_ty)) ; let source_ty = self . monomorphize (source_ty) ; if let ty :: Closure (def_id , args) = * source_ty . kind () { let instance = Instance :: resolve_closure (self . tcx , def_id , args , ty :: ClosureKind :: FnOnce) ; if self . tcx . should_codegen_locally (instance) { self . used_items . push (create_fn_mono_item (self . tcx , instance , span)) ; } } else { bug ! () } } mir :: Rvalue :: ThreadLocalRef (def_id) => { assert ! (self . tcx . is_thread_local_static (def_id)) ; let instance = Instance :: mono (self . tcx , def_id) ; if self . tcx . should_codegen_locally (instance) { trace ! ("collecting thread-local static {:?}" , def_id) ; self . used_items . push (respan (span , MonoItem :: Static (def_id))) ; } } _ => { } } self . super_rvalue (rvalue , location) ; } # [doc = " This does not walk the MIR of the constant as that is not needed for codegen, all we need is"] # [doc = " to ensure that the constant evaluates successfully and walk the result."] # [instrument (skip (self) , level = "debug")] fn visit_const_operand (& mut self , constant : & mir :: ConstOperand < 'tcx > , _location : Location) { let Some (val) = self . eval_constant (constant) else { return } ; collect_const_value (self . tcx , val , self . used_items) ; } fn visit_terminator (& mut self , terminator : & mir :: Terminator < 'tcx > , location : Location) { debug ! ("visiting terminator {:?} @ {:?}" , terminator , location) ; let source = self . body . source_info (location) . span ; let tcx = self . tcx ; let push_mono_lang_item = | this : & mut Self , lang_item : LangItem | { let instance = Instance :: mono (tcx , tcx . require_lang_item (lang_item , source)) ; if tcx . should_codegen_locally (instance) { this . used_items . push (create_fn_mono_item (tcx , instance , source)) ; } } ; match terminator . kind { mir :: TerminatorKind :: Call { ref func , .. } | mir :: TerminatorKind :: TailCall { ref func , .. } => { let callee_ty = func . ty (self . body , tcx) ; self . used_mentioned_items . insert (MentionedItem :: Fn (callee_ty)) ; let callee_ty = self . monomorphize (callee_ty) ; let force_indirect_call = if matches ! (terminator . kind , mir :: TerminatorKind :: TailCall { .. }) && let & ty :: FnDef (def_id , args) = callee_ty . kind () && let instance = ty :: Instance :: expect_resolve (self . tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args , source ,) && instance . def . requires_caller_location (self . tcx) { true } else { false } ; visit_fn_use (self . tcx , callee_ty , ! force_indirect_call , source , & mut self . used_items ,) } mir :: TerminatorKind :: Drop { ref place , .. } => { let ty = place . ty (self . body , self . tcx) . ty ; self . used_mentioned_items . insert (MentionedItem :: Drop (ty)) ; let ty = self . monomorphize (ty) ; visit_drop_use (self . tcx , ty , true , source , self . used_items) ; } mir :: TerminatorKind :: InlineAsm { ref operands , .. } => { for op in operands { match * op { mir :: InlineAsmOperand :: SymFn { ref value } => { let fn_ty = value . const_ . ty () ; self . used_mentioned_items . insert (MentionedItem :: Fn (fn_ty)) ; let fn_ty = self . monomorphize (fn_ty) ; visit_fn_use (self . tcx , fn_ty , false , source , self . used_items) ; } mir :: InlineAsmOperand :: SymStatic { def_id } => { let instance = Instance :: mono (self . tcx , def_id) ; if self . tcx . should_codegen_locally (instance) { trace ! ("collecting asm sym static {:?}" , def_id) ; self . used_items . push (respan (source , MonoItem :: Static (def_id))) ; } } _ => { } } } } mir :: TerminatorKind :: Assert { ref msg , .. } => match & * * msg { mir :: AssertKind :: BoundsCheck { .. } => { push_mono_lang_item (self , LangItem :: PanicBoundsCheck) ; } mir :: AssertKind :: MisalignedPointerDereference { .. } => { push_mono_lang_item (self , LangItem :: PanicMisalignedPointerDereference) ; } mir :: AssertKind :: NullPointerDereference => { push_mono_lang_item (self , LangItem :: PanicNullPointerDereference) ; } mir :: AssertKind :: InvalidEnumConstruction (_) => { push_mono_lang_item (self , LangItem :: PanicInvalidEnumConstruction) ; } _ => { push_mono_lang_item (self , msg . panic_function ()) ; } } , mir :: TerminatorKind :: UnwindTerminate (reason) => { push_mono_lang_item (self , reason . lang_item ()) ; } mir :: TerminatorKind :: Goto { .. } | mir :: TerminatorKind :: SwitchInt { .. } | mir :: TerminatorKind :: UnwindResume | mir :: TerminatorKind :: Return | mir :: TerminatorKind :: Unreachable => { } mir :: TerminatorKind :: CoroutineDrop | mir :: TerminatorKind :: Yield { .. } | mir :: TerminatorKind :: FalseEdge { .. } | mir :: TerminatorKind :: FalseUnwind { .. } => bug ! () , } if let Some (mir :: UnwindAction :: Terminate (reason)) = terminator . unwind () { push_mono_lang_item (self , reason . lang_item ()) ; } self . super_terminator (terminator , location) ; } }}}

macro_rules! visit_drop_use_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visit_drop_use in module {}", module_path!());
    };
}

mkfn!{
    visit_drop_use_introspect!();
    fn visit_drop_use < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , is_direct_call : bool , source : Span , output : & mut MonoItems < 'tcx > ,) { let instance = Instance :: resolve_drop_in_place (tcx , ty) ; visit_instance_use (tcx , instance , is_direct_call , source , output) ; }
}

macro_rules! visit_fn_use_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visit_fn_use in module {}", module_path!());
    };
}

mkfn!{
    visit_fn_use_introspect!();
    # [doc = " For every call of this function in the visitor, make sure there is a matching call in the"] # [doc = " `mentioned_items` pass!"] fn visit_fn_use < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , is_direct_call : bool , source : Span , output : & mut MonoItems < 'tcx > ,) { if let ty :: FnDef (def_id , args) = * ty . kind () { let instance = if is_direct_call { ty :: Instance :: expect_resolve (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args , source ,) } else { match ty :: Instance :: resolve_for_fn_ptr (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args ,) { Some (instance) => instance , _ => bug ! ("failed to resolve instance for {ty}") , } } ; visit_instance_use (tcx , instance , is_direct_call , source , output) ; } }
}

macro_rules! visit_instance_use_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visit_instance_use in module {}", module_path!());
    };
}

mkfn!{
    visit_instance_use_introspect!();
    fn visit_instance_use < 'tcx > (tcx : TyCtxt < 'tcx > , instance : ty :: Instance < 'tcx > , is_direct_call : bool , source : Span , output : & mut MonoItems < 'tcx > ,) { debug ! ("visit_item_use({:?}, is_direct_call={:?})" , instance , is_direct_call) ; if ! tcx . should_codegen_locally (instance) { return ; } if let Some (intrinsic) = tcx . intrinsic (instance . def_id ()) { collect_autodiff_fn (tcx , instance , intrinsic , output) ; if let Some (_requirement) = ValidityRequirement :: from_intrinsic (intrinsic . name) { let def_id = tcx . require_lang_item (LangItem :: PanicNounwind , source) ; let panic_instance = Instance :: mono (tcx , def_id) ; if tcx . should_codegen_locally (panic_instance) { output . push (create_fn_mono_item (tcx , panic_instance , source)) ; } } else if ! intrinsic . must_be_overridden { let instance = ty :: Instance :: new_raw (instance . def_id () , instance . args) ; if tcx . should_codegen_locally (instance) { output . push (create_fn_mono_item (tcx , instance , source)) ; } } } match instance . def { ty :: InstanceKind :: Virtual (..) | ty :: InstanceKind :: Intrinsic (_) => { if ! is_direct_call { bug ! ("{:?} being reified" , instance) ; } } ty :: InstanceKind :: ThreadLocalShim (..) => { bug ! ("{:?} being reified" , instance) ; } ty :: InstanceKind :: DropGlue (_ , None) => { if ! is_direct_call { output . push (create_fn_mono_item (tcx , instance , source)) ; } } ty :: InstanceKind :: DropGlue (_ , Some (_)) | ty :: InstanceKind :: FutureDropPollShim (..) | ty :: InstanceKind :: AsyncDropGlue (_ , _) | ty :: InstanceKind :: AsyncDropGlueCtorShim (_ , _) | ty :: InstanceKind :: VTableShim (..) | ty :: InstanceKind :: ReifyShim (..) | ty :: InstanceKind :: ClosureOnceShim { .. } | ty :: InstanceKind :: ConstructCoroutineInClosureShim { .. } | ty :: InstanceKind :: Item (..) | ty :: InstanceKind :: FnPtrShim (..) | ty :: InstanceKind :: CloneShim (..) | ty :: InstanceKind :: FnPtrAddrShim (..) => { output . push (create_fn_mono_item (tcx , instance , source)) ; } } }
}

macro_rules! should_codegen_locally_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function should_codegen_locally in module {}", module_path!());
    };
}

mkfn!{
    should_codegen_locally_introspect!();
    # [doc = " Returns `true` if we should codegen an instance in the local crate, or returns `false` if we"] # [doc = " can just link to the upstream crate and therefore don't need a mono item."] fn should_codegen_locally < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx >) -> bool { let Some (def_id) = instance . def . def_id_if_not_guaranteed_local_codegen () else { return true ; } ; if tcx . is_foreign_item (def_id) { return false ; } if tcx . def_kind (def_id) . has_codegen_attrs () && matches ! (tcx . codegen_fn_attrs (def_id) . inline , InlineAttr :: Force { .. }) { tcx . dcx () . delayed_bug ("attempt to codegen `#[rustc_force_inline]` item") ; } if def_id . is_local () { return true ; } if tcx . is_reachable_non_generic (def_id) || instance . upstream_monomorphization (tcx) . is_some () { return false ; } if let DefKind :: Static { .. } = tcx . def_kind (def_id) { return false ; } if ! tcx . is_mir_available (def_id) { tcx . dcx () . emit_fatal (NoOptimizedMir { span : tcx . def_span (def_id) , crate_name : tcx . crate_name (def_id . krate) , instance : instance . to_string () , }) ; } true }
}

macro_rules! find_tails_for_unsizing_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_tails_for_unsizing in module {}", module_path!());
    };
}

mkfn!{
    find_tails_for_unsizing_introspect!();
    # [doc = " For a given pair of source and target type that occur in an unsizing coercion,"] # [doc = " this function finds the pair of types that determines the vtable linking"] # [doc = " them."] # [doc = ""] # [doc = " For example, the source type might be `&SomeStruct` and the target type"] # [doc = " might be `&dyn SomeTrait` in a cast like:"] # [doc = ""] # [doc = " ```rust,ignore (not real code)"] # [doc = " let src: &SomeStruct = ...;"] # [doc = " let target = src as &dyn SomeTrait;"] # [doc = " ```"] # [doc = ""] # [doc = " Then the output of this function would be (SomeStruct, SomeTrait) since for"] # [doc = " constructing the `target` wide-pointer we need the vtable for that pair."] # [doc = ""] # [doc = " Things can get more complicated though because there's also the case where"] # [doc = " the unsized type occurs as a field:"] # [doc = ""] # [doc = " ```rust"] # [doc = " struct ComplexStruct<T: ?Sized> {"] # [doc = "    a: u32,"] # [doc = "    b: f64,"] # [doc = "    c: T"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " In this case, if `T` is sized, `&ComplexStruct<T>` is a thin pointer. If `T`"] # [doc = " is unsized, `&SomeStruct` is a wide pointer, and the vtable it points to is"] # [doc = " for the pair of `T` (which is a trait) and the concrete type that `T` was"] # [doc = " originally coerced from:"] # [doc = ""] # [doc = " ```rust,ignore (not real code)"] # [doc = " let src: &ComplexStruct<SomeStruct> = ...;"] # [doc = " let target = src as &ComplexStruct<dyn SomeTrait>;"] # [doc = " ```"] # [doc = ""] # [doc = " Again, we want this `find_vtable_types_for_unsizing()` to provide the pair"] # [doc = " `(SomeStruct, SomeTrait)`."] # [doc = ""] # [doc = " Finally, there is also the case of custom unsizing coercions, e.g., for"] # [doc = " smart pointers such as `Rc` and `Arc`."] fn find_tails_for_unsizing < 'tcx > (tcx : TyCtxtAt < 'tcx > , source_ty : Ty < 'tcx > , target_ty : Ty < 'tcx > ,) -> (Ty < 'tcx > , Ty < 'tcx >) { let typing_env = ty :: TypingEnv :: fully_monomorphized () ; debug_assert ! (! source_ty . has_param () , "{source_ty} should be fully monomorphic") ; debug_assert ! (! target_ty . has_param () , "{target_ty} should be fully monomorphic") ; match (source_ty . kind () , target_ty . kind ()) { (& ty :: Ref (_ , source_pointee , _) , & ty :: Ref (_ , target_pointee , _) | & ty :: RawPtr (target_pointee , _) ,) | (& ty :: RawPtr (source_pointee , _) , & ty :: RawPtr (target_pointee , _)) => { tcx . struct_lockstep_tails_for_codegen (source_pointee , target_pointee , typing_env) } (_ , _) if let Some (source_boxed) = source_ty . boxed_ty () && let Some (target_boxed) = target_ty . boxed_ty () => { tcx . struct_lockstep_tails_for_codegen (source_boxed , target_boxed , typing_env) } (& ty :: Adt (source_adt_def , source_args) , & ty :: Adt (target_adt_def , target_args)) => { assert_eq ! (source_adt_def , target_adt_def) ; let CustomCoerceUnsized :: Struct (coerce_index) = match crate :: custom_coerce_unsize_info (tcx , source_ty , target_ty) { Ok (ccu) => ccu , Err (e) => { let e = Ty :: new_error (tcx . tcx , e) ; return (e , e) ; } } ; let coerce_field = & source_adt_def . non_enum_variant () . fields [coerce_index] ; let source_field = tcx . normalize_erasing_regions (typing_env , coerce_field . ty (* tcx , source_args)) ; let target_field = tcx . normalize_erasing_regions (typing_env , coerce_field . ty (* tcx , target_args)) ; find_tails_for_unsizing (tcx , source_field , target_field) } _ => bug ! ("find_vtable_types_for_unsizing: invalid coercion {:?} -> {:?}" , source_ty , target_ty) , } }
}

macro_rules! create_fn_mono_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_fn_mono_item in module {}", module_path!());
    };
}

mkfn!{
    create_fn_mono_item_introspect!();
    # [instrument (skip (tcx) , level = "debug" , ret)] fn create_fn_mono_item < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , source : Span ,) -> Spanned < MonoItem < 'tcx > > { let def_id = instance . def_id () ; if tcx . sess . opts . unstable_opts . profile_closures && def_id . is_local () && tcx . is_closure_like (def_id) { crate :: util :: dump_closure_profile (tcx , instance) ; } respan (source , MonoItem :: Fn (instance)) }
}

macro_rules! create_mono_items_for_vtable_methods_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_mono_items_for_vtable_methods in module {}", module_path!());
    };
}

mkfn!{
    create_mono_items_for_vtable_methods_introspect!();
    # [doc = " Creates a `MonoItem` for each method that is referenced by the vtable for"] # [doc = " the given trait/impl pair."] fn create_mono_items_for_vtable_methods < 'tcx > (tcx : TyCtxt < 'tcx > , trait_ty : Ty < 'tcx > , impl_ty : Ty < 'tcx > , source : Span , output : & mut MonoItems < 'tcx > ,) { assert ! (! trait_ty . has_escaping_bound_vars () && ! impl_ty . has_escaping_bound_vars ()) ; let ty :: Dynamic (trait_ty , ..) = trait_ty . kind () else { bug ! ("create_mono_items_for_vtable_methods: {trait_ty:?} not a trait type") ; } ; if let Some (principal) = trait_ty . principal () { let trait_ref = tcx . instantiate_bound_regions_with_erased (principal . with_self_ty (tcx , impl_ty)) ; assert ! (! trait_ref . has_escaping_bound_vars ()) ; let entries = tcx . vtable_entries (trait_ref) ; debug ! (? entries) ; let methods = entries . iter () . filter_map (| entry | match entry { VtblEntry :: MetadataDropInPlace | VtblEntry :: MetadataSize | VtblEntry :: MetadataAlign | VtblEntry :: Vacant => None , VtblEntry :: TraitVPtr (_) => { None } VtblEntry :: Method (instance) => { Some (* instance) . filter (| instance | tcx . should_codegen_locally (* instance)) } }) . map (| item | create_fn_mono_item (tcx , item , source)) ; output . extend (methods) ; } if impl_ty . needs_drop (tcx , ty :: TypingEnv :: fully_monomorphized ()) { visit_drop_use (tcx , impl_ty , false , source , output) ; } }
}

macro_rules! collect_alloc_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_alloc in module {}", module_path!());
    };
}

mkfn!{
    collect_alloc_introspect!();
    # [doc = " Scans the CTFE alloc in order to find function pointers and statics that must be monomorphized."] fn collect_alloc < 'tcx > (tcx : TyCtxt < 'tcx > , alloc_id : AllocId , output : & mut MonoItems < 'tcx >) { match tcx . global_alloc (alloc_id) { GlobalAlloc :: Static (def_id) => { assert ! (! tcx . is_thread_local_static (def_id)) ; let instance = Instance :: mono (tcx , def_id) ; if tcx . should_codegen_locally (instance) { trace ! ("collecting static {:?}" , def_id) ; output . push (dummy_spanned (MonoItem :: Static (def_id))) ; } } GlobalAlloc :: Memory (alloc) => { trace ! ("collecting {:?} with {:#?}" , alloc_id , alloc) ; let ptrs = alloc . inner () . provenance () . ptrs () ; if ! ptrs . is_empty () { rustc_data_structures :: stack :: ensure_sufficient_stack (move | | { for & prov in ptrs . values () { collect_alloc (tcx , prov . alloc_id () , output) ; } }) ; } } GlobalAlloc :: Function { instance , .. } => { if tcx . should_codegen_locally (instance) { trace ! ("collecting {:?} with {:#?}" , alloc_id , instance) ; output . push (create_fn_mono_item (tcx , instance , DUMMY_SP)) ; } } GlobalAlloc :: VTable (ty , dyn_ty) => { let alloc_id = tcx . vtable_allocation ((ty , dyn_ty . principal () . map (| principal | tcx . instantiate_bound_regions_with_erased (principal)) ,)) ; collect_alloc (tcx , alloc_id , output) } GlobalAlloc :: TypeId { .. } => { } } }
}

macro_rules! collect_items_of_instance_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_items_of_instance in module {}", module_path!());
    };
}

mkfn!{
    collect_items_of_instance_introspect!();
    # [doc = " Scans the MIR in order to find function calls, closures, and drop-glue."] # [doc = ""] # [doc = " Anything that's found is added to `output`. Furthermore the \"mentioned items\" of the MIR are returned."] # [instrument (skip (tcx) , level = "debug")] fn collect_items_of_instance < 'tcx > (tcx : TyCtxt < 'tcx > , instance : Instance < 'tcx > , mode : CollectionMode ,) -> (MonoItems < 'tcx > , MonoItems < 'tcx >) { tcx . ensure_ok () . check_mono_item (instance) ; let body = tcx . instance_mir (instance . def) ; let mut used_items = MonoItems :: new () ; let mut mentioned_items = MonoItems :: new () ; let mut used_mentioned_items = Default :: default () ; let mut collector = MirUsedCollector { tcx , body , used_items : & mut used_items , used_mentioned_items : & mut used_mentioned_items , instance , } ; if mode == CollectionMode :: UsedItems { if tcx . sess . opts . debuginfo == DebugInfo :: Full { for var_debug_info in & body . var_debug_info { collector . visit_var_debug_info (var_debug_info) ; } } for (bb , data) in traversal :: mono_reachable (body , tcx , instance) { collector . visit_basic_block_data (bb , data) } } for const_op in body . required_consts () { if let Some (val) = collector . eval_constant (const_op) { collect_const_value (tcx , val , & mut mentioned_items) ; } } for item in body . mentioned_items () { if ! collector . used_mentioned_items . contains (& item . node) { let item_mono = collector . monomorphize (item . node) ; visit_mentioned_item (tcx , & item_mono , item . span , & mut mentioned_items) ; } } (used_items , mentioned_items) }
}

macro_rules! items_of_instance_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function items_of_instance in module {}", module_path!());
    };
}

mkfn!{
    items_of_instance_introspect!();
    fn items_of_instance < 'tcx > (tcx : TyCtxt < 'tcx > , (instance , mode) : (Instance < 'tcx > , CollectionMode) ,) -> (& 'tcx [Spanned < MonoItem < 'tcx > >] , & 'tcx [Spanned < MonoItem < 'tcx > >]) { let (used_items , mentioned_items) = collect_items_of_instance (tcx , instance , mode) ; let used_items = tcx . arena . alloc_from_iter (used_items) ; let mentioned_items = tcx . arena . alloc_from_iter (mentioned_items) ; (used_items , mentioned_items) }
}

macro_rules! visit_mentioned_item_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function visit_mentioned_item in module {}", module_path!());
    };
}

mkfn!{
    visit_mentioned_item_introspect!();
    # [doc = " `item` must be already monomorphized."] # [instrument (skip (tcx , span , output) , level = "debug")] fn visit_mentioned_item < 'tcx > (tcx : TyCtxt < 'tcx > , item : & MentionedItem < 'tcx > , span : Span , output : & mut MonoItems < 'tcx > ,) { match * item { MentionedItem :: Fn (ty) => { if let ty :: FnDef (def_id , args) = * ty . kind () { let instance = Instance :: expect_resolve (tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , args , span ,) ; visit_instance_use (tcx , instance , true , span , output) ; } } MentionedItem :: Drop (ty) => { visit_drop_use (tcx , ty , true , span , output) ; } MentionedItem :: UnsizeCast { source_ty , target_ty } => { let (source_ty , target_ty) = find_tails_for_unsizing (tcx . at (span) , source_ty , target_ty) ; if target_ty . is_trait () && ! source_ty . is_trait () { create_mono_items_for_vtable_methods (tcx , target_ty , source_ty , span , output) ; } } MentionedItem :: Closure (source_ty) => { if let ty :: Closure (def_id , args) = * source_ty . kind () { let instance = Instance :: resolve_closure (tcx , def_id , args , ty :: ClosureKind :: FnOnce) ; if tcx . should_codegen_locally (instance) { output . push (create_fn_mono_item (tcx , instance , span)) ; } } else { bug ! () } } } }
}

macro_rules! collect_const_value_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_const_value in module {}", module_path!());
    };
}

mkfn!{
    collect_const_value_introspect!();
    # [instrument (skip (tcx , output) , level = "debug")] fn collect_const_value < 'tcx > (tcx : TyCtxt < 'tcx > , value : mir :: ConstValue , output : & mut MonoItems < 'tcx > ,) { match value { mir :: ConstValue :: Scalar (Scalar :: Ptr (ptr , _size)) => { collect_alloc (tcx , ptr . provenance . alloc_id () , output) } mir :: ConstValue :: Indirect { alloc_id , .. } | mir :: ConstValue :: Slice { alloc_id , meta : _ } => collect_alloc (tcx , alloc_id , output) , _ => { } } }
}

macro_rules! collect_roots_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_roots in module {}", module_path!());
    };
}

mkfn!{
    collect_roots_introspect!();
    # [instrument (skip (tcx , mode) , level = "debug")] fn collect_roots (tcx : TyCtxt < '_ > , mode : MonoItemCollectionStrategy) -> Vec < MonoItem < '_ > > { debug ! ("collecting roots") ; let mut roots = MonoItems :: new () ; { let entry_fn = tcx . entry_fn (()) ; debug ! ("collect_roots: entry_fn = {:?}" , entry_fn) ; let mut collector = RootCollector { tcx , strategy : mode , entry_fn , output : & mut roots } ; let crate_items = tcx . hir_crate_items (()) ; for id in crate_items . free_items () { collector . process_item (id) ; } for id in crate_items . impl_items () { collector . process_impl_item (id) ; } for id in crate_items . nested_bodies () { collector . process_nested_body (id) ; } collector . push_extra_entry_roots () ; } roots . into_iter () . filter_map (| Spanned { node : mono_item , .. } | { mono_item . is_instantiable (tcx) . then_some (mono_item) }) . collect () }
}
mkitem!{mkstruct!{struct RootCollector < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , strategy : MonoItemCollectionStrategy , output : & 'a mut MonoItems < 'tcx > , entry_fn : Option < (DefId , EntryFnType) > , }}}
mkitem!{mkimpl!{impl < 'v > RootCollector < '_ , 'v > { fn process_item (& mut self , id : hir :: ItemId) { match self . tcx . def_kind (id . owner_id) { DefKind :: Enum | DefKind :: Struct | DefKind :: Union => { if self . strategy == MonoItemCollectionStrategy :: Eager && ! self . tcx . generics_of (id . owner_id) . requires_monomorphization (self . tcx) { debug ! ("RootCollector: ADT drop-glue for `{id:?}`" ,) ; let id_args = ty :: GenericArgs :: for_item (self . tcx , id . owner_id . to_def_id () , | param , _ | { match param . kind { GenericParamDefKind :: Lifetime => { self . tcx . lifetimes . re_erased . into () } GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { unreachable ! ("`own_requires_monomorphization` check means that \
                                we should have no type/const params") } } }) ; if self . tcx . instantiate_and_check_impossible_predicates ((id . owner_id . to_def_id () , id_args ,)) { return ; } let ty = self . tcx . type_of (id . owner_id . to_def_id ()) . instantiate (self . tcx , id_args) ; assert ! (! ty . has_non_region_param ()) ; visit_drop_use (self . tcx , ty , true , DUMMY_SP , self . output) ; } } DefKind :: GlobalAsm => { debug ! ("RootCollector: ItemKind::GlobalAsm({})" , self . tcx . def_path_str (id . owner_id)) ; self . output . push (dummy_spanned (MonoItem :: GlobalAsm (id))) ; } DefKind :: Static { .. } => { let def_id = id . owner_id . to_def_id () ; debug ! ("RootCollector: ItemKind::Static({})" , self . tcx . def_path_str (def_id)) ; self . output . push (dummy_spanned (MonoItem :: Static (def_id))) ; } DefKind :: Const => { if self . strategy == MonoItemCollectionStrategy :: Eager { if ! self . tcx . generics_of (id . owner_id) . own_requires_monomorphization () && let Ok (val) = self . tcx . const_eval_poly (id . owner_id . to_def_id ()) { collect_const_value (self . tcx , val , self . output) ; } } } DefKind :: Impl { .. } => { if self . strategy == MonoItemCollectionStrategy :: Eager { create_mono_items_for_default_impls (self . tcx , id , self . output) ; } } DefKind :: Fn => { self . push_if_root (id . owner_id . def_id) ; } _ => { } } } fn process_impl_item (& mut self , id : hir :: ImplItemId) { if matches ! (self . tcx . def_kind (id . owner_id) , DefKind :: AssocFn) { self . push_if_root (id . owner_id . def_id) ; } } fn process_nested_body (& mut self , def_id : LocalDefId) { match self . tcx . def_kind (def_id) { DefKind :: Closure => { let is_pub_fn_coroutine = match * self . tcx . type_of (def_id) . instantiate_identity () . kind () { ty :: Coroutine (cor_id , _args) => { let tcx = self . tcx ; let parent_id = tcx . parent (cor_id) ; tcx . def_kind (parent_id) == DefKind :: Fn && tcx . asyncness (parent_id) . is_async () && tcx . visibility (parent_id) . is_public () } ty :: Closure (..) | ty :: CoroutineClosure (..) => false , _ => unreachable ! () , } ; if (self . strategy == MonoItemCollectionStrategy :: Eager || is_pub_fn_coroutine) && ! self . tcx . generics_of (self . tcx . typeck_root_def_id (def_id . to_def_id ())) . requires_monomorphization (self . tcx) { let instance = match * self . tcx . type_of (def_id) . instantiate_identity () . kind () { ty :: Closure (def_id , args) | ty :: Coroutine (def_id , args) | ty :: CoroutineClosure (def_id , args) => { Instance :: new_raw (def_id , self . tcx . erase_and_anonymize_regions (args)) } _ => unreachable ! () , } ; let Ok (instance) = self . tcx . try_normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , instance ,) else { return ; } ; let mono_item = create_fn_mono_item (self . tcx , instance , DUMMY_SP) ; if mono_item . node . is_instantiable (self . tcx) { self . output . push (mono_item) ; } } } _ => { } } } fn is_root (& self , def_id : LocalDefId) -> bool { ! self . tcx . generics_of (def_id) . requires_monomorphization (self . tcx) && match self . strategy { MonoItemCollectionStrategy :: Eager => { ! matches ! (self . tcx . codegen_fn_attrs (def_id) . inline , InlineAttr :: Force { .. }) } MonoItemCollectionStrategy :: Lazy => { self . entry_fn . and_then (| (id , _) | id . as_local ()) == Some (def_id) || self . tcx . is_reachable_non_generic (def_id) || self . tcx . codegen_fn_attrs (def_id) . flags . contains (CodegenFnAttrFlags :: RUSTC_STD_INTERNAL_SYMBOL) } } } # [doc = " If `def_id` represents a root, pushes it onto the list of"] # [doc = " outputs. (Note that all roots must be monomorphic.)"] # [instrument (skip (self) , level = "debug")] fn push_if_root (& mut self , def_id : LocalDefId) { if self . is_root (def_id) { debug ! ("found root") ; let instance = Instance :: mono (self . tcx , def_id . to_def_id ()) ; self . output . push (create_fn_mono_item (self . tcx , instance , DUMMY_SP)) ; } } # [doc = " As a special case, when/if we encounter the"] # [doc = " `main()` function, we also have to generate a"] # [doc = " monomorphized copy of the start lang item based on"] # [doc = " the return type of `main`. This is not needed when"] # [doc = " the user writes their own `start` manually."] fn push_extra_entry_roots (& mut self) { let Some ((main_def_id , EntryFnType :: Main { .. })) = self . entry_fn else { return ; } ; let main_instance = Instance :: mono (self . tcx , main_def_id) ; if self . tcx . should_codegen_locally (main_instance) { self . output . push (create_fn_mono_item (self . tcx , main_instance , self . tcx . def_span (main_def_id) ,)) ; } let Some (start_def_id) = self . tcx . lang_items () . start_fn () else { self . tcx . dcx () . emit_fatal (errors :: StartNotFound) ; } ; let main_ret_ty = self . tcx . fn_sig (main_def_id) . no_bound_vars () . unwrap () . output () ; let main_ret_ty = self . tcx . normalize_erasing_regions (ty :: TypingEnv :: fully_monomorphized () , main_ret_ty . no_bound_vars () . unwrap () ,) ; let start_instance = Instance :: expect_resolve (self . tcx , ty :: TypingEnv :: fully_monomorphized () , start_def_id , self . tcx . mk_args (& [main_ret_ty . into ()]) , DUMMY_SP ,) ; self . output . push (create_fn_mono_item (self . tcx , start_instance , DUMMY_SP)) ; } }}}

macro_rules! create_mono_items_for_default_impls_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_mono_items_for_default_impls in module {}", module_path!());
    };
}

mkfn!{
    create_mono_items_for_default_impls_introspect!();
    # [instrument (level = "debug" , skip (tcx , output))] fn create_mono_items_for_default_impls < 'tcx > (tcx : TyCtxt < 'tcx > , item : hir :: ItemId , output : & mut MonoItems < 'tcx > ,) { let Some (impl_) = tcx . impl_trait_header (item . owner_id) else { return ; } ; if matches ! (impl_ . polarity , ty :: ImplPolarity :: Negative) { return ; } if tcx . generics_of (item . owner_id) . own_requires_monomorphization () { return ; } let only_region_params = | param : & ty :: GenericParamDef , _ : & _ | match param . kind { GenericParamDefKind :: Lifetime => tcx . lifetimes . re_erased . into () , GenericParamDefKind :: Type { .. } | GenericParamDefKind :: Const { .. } => { unreachable ! ("`own_requires_monomorphization` check means that \
                we should have no type/const params") } } ; let impl_args = GenericArgs :: for_item (tcx , item . owner_id . to_def_id () , only_region_params) ; let trait_ref = impl_ . trait_ref . instantiate (tcx , impl_args) ; if tcx . instantiate_and_check_impossible_predicates ((item . owner_id . to_def_id () , impl_args)) { return ; } let typing_env = ty :: TypingEnv :: fully_monomorphized () ; let trait_ref = tcx . normalize_erasing_regions (typing_env , trait_ref) ; let overridden_methods = tcx . impl_item_implementor_ids (item . owner_id) ; for method in tcx . provided_trait_methods (trait_ref . def_id) { if overridden_methods . contains_key (& method . def_id) { continue ; } if tcx . generics_of (method . def_id) . own_requires_monomorphization () { continue ; } let args = trait_ref . args . extend_to (tcx , method . def_id , only_region_params) ; let instance = ty :: Instance :: expect_resolve (tcx , typing_env , method . def_id , args , DUMMY_SP) ; let mono_item = create_fn_mono_item (tcx , instance , DUMMY_SP) ; if mono_item . node . is_instantiable (tcx) && tcx . should_codegen_locally (instance) { output . push (mono_item) ; } } }
}

macro_rules! collect_crate_mono_items_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function collect_crate_mono_items in module {}", module_path!());
    };
}

mkfn!{
    collect_crate_mono_items_introspect!();
    # [instrument (skip (tcx , strategy) , level = "debug")] pub (crate) fn collect_crate_mono_items < 'tcx > (tcx : TyCtxt < 'tcx > , strategy : MonoItemCollectionStrategy ,) -> (Vec < MonoItem < 'tcx > > , UsageMap < 'tcx >) { let _prof_timer = tcx . prof . generic_activity ("monomorphization_collector") ; let roots = tcx . sess . time ("monomorphization_collector_root_collections" , | | collect_roots (tcx , strategy)) ; debug ! ("building mono item graph, beginning at roots") ; let state = SharedState { visited : MTLock :: new (UnordSet :: default ()) , mentioned : MTLock :: new (UnordSet :: default ()) , usage_map : MTLock :: new (UsageMap :: new ()) , } ; let recursion_limit = tcx . recursion_limit () ; tcx . sess . time ("monomorphization_collector_graph_walk" , | | { par_for_each_in (roots , | root | { collect_items_root (tcx , dummy_spanned (* root) , & state , recursion_limit) ; }) ; }) ; let mono_items = tcx . with_stable_hashing_context (move | ref hcx | { state . visited . into_inner () . into_sorted (hcx , true) }) ; (mono_items , state . usage_map . into_inner ()) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { providers . hooks . should_codegen_locally = should_codegen_locally ; providers . items_of_instance = items_of_instance ; }
}