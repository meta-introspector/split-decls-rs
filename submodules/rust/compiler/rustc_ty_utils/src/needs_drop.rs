mkuse!{use rustc_data_structures :: fx :: FxHashSet ;}
mkuse!{use rustc_hir :: def_id :: DefId ;}
mkuse!{use rustc_hir :: limit :: Limit ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: query :: Providers ;}
mkuse!{use rustc_middle :: ty :: util :: { AlwaysRequiresDrop , needs_drop_components } ;}
mkuse!{use rustc_middle :: ty :: { self , EarlyBinder , GenericArgsRef , Ty , TyCtxt } ;}
mkuse!{use rustc_span :: sym ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: errors :: NeedsDropOverflow ;}
mkitem!{type NeedsDropResult < T > = Result < T , AlwaysRequiresDrop > ;}

macro_rules! needs_drop_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function needs_drop_raw in module {}", module_path!());
    };
}

mkfn!{
    needs_drop_raw_introspect!();
    fn needs_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { let adt_has_dtor = | adt_def : ty :: AdtDef < 'tcx > | adt_def . destructor (tcx) . map (| _ | DtorType :: Significant) ; let res = drop_tys_helper (tcx , query . value , query . typing_env , adt_has_dtor , false , false) . filter (filter_array_elements (tcx , query . typing_env)) . next () . is_some () ; debug ! ("needs_drop_raw({:?}) = {:?}" , query , res) ; res }
}

macro_rules! needs_async_drop_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function needs_async_drop_raw in module {}", module_path!());
    };
}

mkfn!{
    needs_async_drop_raw_introspect!();
    fn needs_async_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { let adt_has_async_dtor = | adt_def : ty :: AdtDef < 'tcx > | adt_def . async_destructor (tcx) . map (| _ | DtorType :: Significant) ; let res = drop_tys_helper (tcx , query . value , query . typing_env , adt_has_async_dtor , false , false) . filter (filter_array_elements_async (tcx , query . typing_env)) . next () . is_some () ; debug ! ("needs_async_drop_raw({:?}) = {:?}" , query , res) ; res }
}

macro_rules! filter_array_elements_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filter_array_elements in module {}", module_path!());
    };
}

mkfn!{
    filter_array_elements_introspect!();
    # [doc = " HACK: in order to not mistakenly assume that `[PhantomData<T>; N]` requires drop glue"] # [doc = " we check the element type for drop glue. The correct fix would be looking at the"] # [doc = " entirety of the code around `needs_drop_components` and this file and come up with"] # [doc = " logic that is easier to follow while not repeating any checks that may thus diverge."] fn filter_array_elements < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> impl Fn (& Result < Ty < 'tcx > , AlwaysRequiresDrop >) -> bool { move | ty | match ty { Ok (ty) => match * ty . kind () { ty :: Array (elem , _) => tcx . needs_drop_raw (typing_env . as_query_input (elem)) , _ => true , } , Err (AlwaysRequiresDrop) => true , } }
}

macro_rules! filter_array_elements_async_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function filter_array_elements_async in module {}", module_path!());
    };
}

mkfn!{
    filter_array_elements_async_introspect!();
    fn filter_array_elements_async < 'tcx > (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > ,) -> impl Fn (& Result < Ty < 'tcx > , AlwaysRequiresDrop >) -> bool { move | ty | match ty { Ok (ty) => match * ty . kind () { ty :: Array (elem , _) => tcx . needs_async_drop_raw (typing_env . as_query_input (elem)) , _ => true , } , Err (AlwaysRequiresDrop) => true , } }
}

macro_rules! has_significant_drop_raw_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function has_significant_drop_raw in module {}", module_path!());
    };
}

mkfn!{
    has_significant_drop_raw_introspect!();
    fn has_significant_drop_raw < 'tcx > (tcx : TyCtxt < 'tcx > , query : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> bool { let res = drop_tys_helper (tcx , query . value , query . typing_env , adt_consider_insignificant_dtor (tcx) , true , false ,) . filter (filter_array_elements (tcx , query . typing_env)) . next () . is_some () ; debug ! ("has_significant_drop_raw({:?}) = {:?}" , query , res) ; res }
}
mkitem!{mkstruct!{struct NeedsDropTypes < 'tcx , F > { tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , query_ty : Ty < 'tcx > , seen_tys : FxHashSet < Ty < 'tcx > > , # [doc = " A stack of types left to process, and the recursion depth when we"] # [doc = " pushed that type. Each round, we pop something from the stack and check"] # [doc = " if it needs drop. If the result depends on whether some other types"] # [doc = " need drop we push them onto the stack."] unchecked_tys : Vec < (Ty < 'tcx > , usize) > , recursion_limit : Limit , adt_components : F , # [doc = " Set this to true if an exhaustive list of types involved in"] # [doc = " drop obligation is requested."] exhaustive : bool , }}}
mkitem!{mkimpl!{impl < 'tcx , F > NeedsDropTypes < 'tcx , F > { fn new (tcx : TyCtxt < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , ty : Ty < 'tcx > , exhaustive : bool , adt_components : F ,) -> Self { let mut seen_tys = FxHashSet :: default () ; seen_tys . insert (ty) ; Self { tcx , typing_env , seen_tys , query_ty : ty , unchecked_tys : vec ! [(ty , 0)] , recursion_limit : tcx . recursion_limit () , adt_components , exhaustive , } } # [doc = " Called when `ty` is found to always require drop."] # [doc = " If the exhaustive flag is true, then `Ok(ty)` is returned like any other type."] # [doc = " Otherwise, `Err(AlwaysRequireDrop)` is returned, which will cause iteration to abort."] fn always_drop_component (& self , ty : Ty < 'tcx >) -> NeedsDropResult < Ty < 'tcx > > { if self . exhaustive { Ok (ty) } else { Err (AlwaysRequiresDrop) } } }}}
mkitem!{mkimpl!{impl < 'tcx , F , I > Iterator for NeedsDropTypes < 'tcx , F > where F : Fn (ty :: AdtDef < 'tcx > , GenericArgsRef < 'tcx >) -> NeedsDropResult < I > , I : Iterator < Item = Ty < 'tcx > > , { type Item = NeedsDropResult < Ty < 'tcx > > ; # [instrument (level = "debug" , skip (self) , ret)] fn next (& mut self) -> Option < NeedsDropResult < Ty < 'tcx > > > { let tcx = self . tcx ; while let Some ((ty , level)) = self . unchecked_tys . pop () { debug ! (? ty , "needs_drop_components: inspect") ; if ! self . recursion_limit . value_within_limit (level) { debug ! ("needs_drop_components: recursion limit exceeded") ; tcx . dcx () . emit_err (NeedsDropOverflow { query_ty : self . query_ty }) ; return Some (self . always_drop_component (ty)) ; } let components = match needs_drop_components (tcx , ty) { Err (AlwaysRequiresDrop) => return Some (self . always_drop_component (ty)) , Ok (components) => components , } ; debug ! ("needs_drop_components({:?}) = {:?}" , ty , components) ; let queue_type = move | this : & mut Self , component : Ty < 'tcx > | { if this . seen_tys . insert (component) { this . unchecked_tys . push ((component , level + 1)) ; } } ; for component in components { match * component . kind () { ty :: Coroutine (def_id , args) => { if self . exhaustive { for upvar in args . as_coroutine () . upvar_tys () { queue_type (self , upvar) ; } queue_type (self , args . as_coroutine () . resume_ty ()) ; if let Some (witness) = tcx . mir_coroutine_witnesses (def_id) { for field_ty in & witness . field_tys { queue_type (self , EarlyBinder :: bind (field_ty . ty) . instantiate (tcx , args) ,) ; } } } else { return Some (self . always_drop_component (ty)) ; } } ty :: CoroutineWitness (..) => { unreachable ! ("witness should be handled in parent") ; } ty :: UnsafeBinder (bound_ty) => { let ty = self . tcx . instantiate_bound_regions_with_erased (bound_ty . into ()) ; queue_type (self , ty) ; } _ if tcx . type_is_copy_modulo_regions (self . typing_env , component) => { } ty :: Closure (_ , args) => { for upvar in args . as_closure () . upvar_tys () { queue_type (self , upvar) ; } } ty :: CoroutineClosure (_ , args) => { for upvar in args . as_coroutine_closure () . upvar_tys () { queue_type (self , upvar) ; } } ty :: Adt (adt_def , args) => { let tys = match (self . adt_components) (adt_def , args) { Err (AlwaysRequiresDrop) => { return Some (self . always_drop_component (ty)) ; } Ok (tys) => tys , } ; for required_ty in tys { let required = tcx . try_normalize_erasing_regions (self . typing_env , required_ty) . unwrap_or (required_ty) ; queue_type (self , required) ; } } ty :: Alias (..) | ty :: Array (..) | ty :: Placeholder (_) | ty :: Param (_) => { if ty == component { return Some (Ok (component)) ; } else { queue_type (self , component) ; } } ty :: Foreign (_) | ty :: Dynamic (..) => { debug ! ("needs_drop_components: foreign or dynamic") ; return Some (self . always_drop_component (ty)) ; } ty :: Bool | ty :: Char | ty :: Int (_) | ty :: Uint (_) | ty :: Float (_) | ty :: Str | ty :: Slice (_) | ty :: Ref (..) | ty :: RawPtr (..) | ty :: FnDef (..) | ty :: Pat (..) | ty :: FnPtr (..) | ty :: Tuple (_) | ty :: Bound (..) | ty :: Never | ty :: Infer (_) | ty :: Error (_) => { bug ! ("unexpected type returned by `needs_drop_components`: {component}") } } } } None } }}}
mkitem!{mkenum!{enum DtorType { # [doc = " Type has a `Drop` but it is considered insignificant."] # [doc = " Check the query `adt_significant_drop_tys` for understanding"] # [doc = " \"significant\" / \"insignificant\"."] Insignificant , # [doc = " Type has a `Drop` implantation."] Significant , }}}

macro_rules! drop_tys_helper_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function drop_tys_helper in module {}", module_path!());
    };
}

mkfn!{
    drop_tys_helper_introspect!();
    fn drop_tys_helper < 'tcx > (tcx : TyCtxt < 'tcx > , ty : Ty < 'tcx > , typing_env : ty :: TypingEnv < 'tcx > , adt_has_dtor : impl Fn (ty :: AdtDef < 'tcx >) -> Option < DtorType > , only_significant : bool , exhaustive : bool ,) -> impl Iterator < Item = NeedsDropResult < Ty < 'tcx > > > { fn with_query_cache < 'tcx > (tcx : TyCtxt < 'tcx > , iter : impl IntoIterator < Item = Ty < 'tcx > > ,) -> NeedsDropResult < Vec < Ty < 'tcx > > > { iter . into_iter () . try_fold (Vec :: new () , | mut vec , subty | { match subty . kind () { ty :: Adt (adt_id , args) => { for subty in tcx . adt_drop_tys (adt_id . did ()) ? { vec . push (EarlyBinder :: bind (subty) . instantiate (tcx , args)) ; } } _ => vec . push (subty) , } ; Ok (vec) }) } let adt_components = move | adt_def : ty :: AdtDef < 'tcx > , args : GenericArgsRef < 'tcx > | { if adt_def . is_manually_drop () { debug ! ("drop_tys_helper: `{:?}` is manually drop" , adt_def) ; Ok (Vec :: new ()) } else if let Some (dtor_info) = adt_has_dtor (adt_def) { match dtor_info { DtorType :: Significant => { debug ! ("drop_tys_helper: `{:?}` implements `Drop`" , adt_def) ; Err (AlwaysRequiresDrop) } DtorType :: Insignificant => { debug ! ("drop_tys_helper: `{:?}` drop is insignificant" , adt_def) ; Ok (args . types () . collect ()) } } } else if adt_def . is_union () { debug ! ("drop_tys_helper: `{:?}` is a union" , adt_def) ; Ok (Vec :: new ()) } else { let field_tys = adt_def . all_fields () . map (| field | { let r = tcx . type_of (field . did) . instantiate (tcx , args) ; debug ! ("drop_tys_helper: Instantiate into {:?} with {:?} getting {:?}" , field , args , r) ; r }) ; if only_significant { Ok (field_tys . collect ()) } else { with_query_cache (tcx , field_tys) } } . map (| v | v . into_iter ()) } ; NeedsDropTypes :: new (tcx , typing_env , ty , exhaustive , adt_components) }
}

macro_rules! adt_consider_insignificant_dtor_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_consider_insignificant_dtor in module {}", module_path!());
    };
}

mkfn!{
    adt_consider_insignificant_dtor_introspect!();
    fn adt_consider_insignificant_dtor < 'tcx > (tcx : TyCtxt < 'tcx > ,) -> impl Fn (ty :: AdtDef < 'tcx >) -> Option < DtorType > { move | adt_def : ty :: AdtDef < 'tcx > | { let is_marked_insig = tcx . has_attr (adt_def . did () , sym :: rustc_insignificant_dtor) ; if is_marked_insig { Some (DtorType :: Insignificant) } else if adt_def . destructor (tcx) . is_some () { Some (DtorType :: Significant) } else { None } } }
}

macro_rules! adt_drop_tys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_drop_tys in module {}", module_path!());
    };
}

mkfn!{
    adt_drop_tys_introspect!();
    fn adt_drop_tys < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId ,) -> Result < & 'tcx ty :: List < Ty < 'tcx > > , AlwaysRequiresDrop > { let adt_has_dtor = | adt_def : ty :: AdtDef < 'tcx > | adt_def . destructor (tcx) . map (| _ | DtorType :: Significant) ; drop_tys_helper (tcx , tcx . type_of (def_id) . instantiate_identity () , ty :: TypingEnv :: non_body_analysis (tcx , def_id) , adt_has_dtor , false , false ,) . collect :: < Result < Vec < _ > , _ > > () . map (| components | tcx . mk_type_list (& components)) }
}

macro_rules! adt_async_drop_tys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_async_drop_tys in module {}", module_path!());
    };
}

mkfn!{
    adt_async_drop_tys_introspect!();
    fn adt_async_drop_tys < 'tcx > (tcx : TyCtxt < 'tcx > , def_id : DefId ,) -> Result < & 'tcx ty :: List < Ty < 'tcx > > , AlwaysRequiresDrop > { let adt_has_dtor = | adt_def : ty :: AdtDef < 'tcx > | adt_def . async_destructor (tcx) . map (| _ | DtorType :: Significant) ; drop_tys_helper (tcx , tcx . type_of (def_id) . instantiate_identity () , ty :: TypingEnv :: non_body_analysis (tcx , def_id) , adt_has_dtor , false , false ,) . collect :: < Result < Vec < _ > , _ > > () . map (| components | tcx . mk_type_list (& components)) }
}

macro_rules! adt_significant_drop_tys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function adt_significant_drop_tys in module {}", module_path!());
    };
}

mkfn!{
    adt_significant_drop_tys_introspect!();
    fn adt_significant_drop_tys (tcx : TyCtxt < '_ > , def_id : DefId ,) -> Result < & ty :: List < Ty < '_ > > , AlwaysRequiresDrop > { drop_tys_helper (tcx , tcx . type_of (def_id) . instantiate_identity () , ty :: TypingEnv :: non_body_analysis (tcx , def_id) , adt_consider_insignificant_dtor (tcx) , true , false ,) . collect :: < Result < Vec < _ > , _ > > () . map (| components | tcx . mk_type_list (& components)) }
}

macro_rules! list_significant_drop_tys_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function list_significant_drop_tys in module {}", module_path!());
    };
}

mkfn!{
    list_significant_drop_tys_introspect!();
    # [instrument (level = "debug" , skip (tcx) , ret)] fn list_significant_drop_tys < 'tcx > (tcx : TyCtxt < 'tcx > , key : ty :: PseudoCanonicalInput < 'tcx , Ty < 'tcx > > ,) -> & 'tcx ty :: List < Ty < 'tcx > > { tcx . mk_type_list (& drop_tys_helper (tcx , key . value , key . typing_env , adt_consider_insignificant_dtor (tcx) , true , true ,) . filter_map (| res | res . ok ()) . collect :: < Vec < _ > > () ,) }
}

macro_rules! provide_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function provide in module {}", module_path!());
    };
}

mkfn!{
    provide_introspect!();
    pub (crate) fn provide (providers : & mut Providers) { * providers = Providers { needs_drop_raw , needs_async_drop_raw , has_significant_drop_raw , adt_drop_tys , adt_async_drop_tys , adt_significant_drop_tys , list_significant_drop_tys , .. * providers } ; }
}