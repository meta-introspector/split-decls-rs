mkuse!{use std :: iter ;}
mkuse!{use rustc_index :: IndexVec ;}
mkuse!{use rustc_index :: bit_set :: DenseBitSet ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: mir :: { Body , Local , UnwindTerminateReason , traversal } ;}
mkuse!{use rustc_middle :: ty :: layout :: { FnAbiOf , HasTyCtxt , HasTypingEnv , TyAndLayout } ;}
mkuse!{use rustc_middle :: ty :: { self , Instance , Ty , TyCtxt , TypeFoldable , TypeVisitableExt } ;}
mkuse!{use rustc_middle :: { bug , mir , span_bug } ;}
mkuse!{use rustc_target :: callconv :: { FnAbi , PassMode } ;}
mkuse!{use tracing :: { debug , instrument } ;}
mkuse!{use crate :: base ;}
mkuse!{use crate :: traits :: * ;}
mkmod!{analyze, { 
                getname!(analyze);
                getsrc!(analyze);
                getpath!(analyze);
                get_deps!(analyze);
                get_crates!(analyze);
                mkinclude!(analyze);
                 
            }}
mkmod!{block, { 
                getname!(block);
                getsrc!(block);
                getpath!(block);
                get_deps!(block);
                get_crates!(block);
                mkinclude!(block);
                 
            }}
mkmod!{constant, { 
                getname!(constant);
                getsrc!(constant);
                getpath!(constant);
                get_deps!(constant);
                get_crates!(constant);
                mkinclude!(constant);
                 
            }}
mkmod!{coverageinfo, { 
                getname!(coverageinfo);
                getsrc!(coverageinfo);
                getpath!(coverageinfo);
                get_deps!(coverageinfo);
                get_crates!(coverageinfo);
                mkinclude!(coverageinfo);
                 
            }}
mkmod!{debuginfo, { 
                getname!(debuginfo);
                getsrc!(debuginfo);
                getpath!(debuginfo);
                get_deps!(debuginfo);
                get_crates!(debuginfo);
                mkinclude!(debuginfo);
                 
            }}
mkmod!{intrinsic, { 
                getname!(intrinsic);
                getsrc!(intrinsic);
                getpath!(intrinsic);
                get_deps!(intrinsic);
                get_crates!(intrinsic);
                mkinclude!(intrinsic);
                 
            }}
mkmod!{locals, { 
                getname!(locals);
                getsrc!(locals);
                getpath!(locals);
                get_deps!(locals);
                get_crates!(locals);
                mkinclude!(locals);
                 
            }}
mkmod!{naked_asm, { 
                getname!(naked_asm);
                getsrc!(naked_asm);
                getpath!(naked_asm);
                get_deps!(naked_asm);
                get_crates!(naked_asm);
                mkinclude!(naked_asm);
                 
            }}
mkmod!{operand, { 
                getname!(operand);
                getsrc!(operand);
                getpath!(operand);
                get_deps!(operand);
                get_crates!(operand);
                mkinclude!(operand);
                 
            }}
mkmod!{place, { 
                getname!(place);
                getsrc!(place);
                getpath!(place);
                get_deps!(place);
                get_crates!(place);
                mkinclude!(place);
                 
            }}
mkmod!{rvalue, { 
                getname!(rvalue);
                getsrc!(rvalue);
                getpath!(rvalue);
                get_deps!(rvalue);
                get_crates!(rvalue);
                mkinclude!(rvalue);
                 
            }}
mkmod!{statement, { 
                getname!(statement);
                getsrc!(statement);
                getpath!(statement);
                get_deps!(statement);
                get_crates!(statement);
                mkinclude!(statement);
                 
            }}
mkuse!{pub use self :: block :: store_cast ;}
mkuse!{use self :: debuginfo :: { FunctionDebugContext , PerLocalVarDebugInfo } ;}
mkuse!{use self :: operand :: { OperandRef , OperandValue } ;}
mkuse!{use self :: place :: PlaceRef ;}
mkitem!{mkenum!{enum CachedLlbb < T > { # [doc = " Nothing created yet."] None , # [doc = " Has been created."] Some (T) , # [doc = " Nothing created yet, and nothing should be."] Skip , }}}
mkitem!{type PerLocalVarDebugInfoIndexVec < 'tcx , V > = IndexVec < mir :: Local , Vec < PerLocalVarDebugInfo < 'tcx , V > > > ;}
mkitem!{mkstruct!{# [doc = " Master context for codegenning from MIR."] pub struct FunctionCx < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > { instance : Instance < 'tcx > , mir : & 'tcx mir :: Body < 'tcx > , debug_context : Option < FunctionDebugContext < 'tcx , Bx :: DIScope , Bx :: DILocation > > , llfn : Bx :: Function , cx : & 'a Bx :: CodegenCx , fn_abi : & 'tcx FnAbi < 'tcx , Ty < 'tcx > > , # [doc = " When unwinding is initiated, we have to store this personality"] # [doc = " value somewhere so that we can load it and re-use it in the"] # [doc = " resume instruction. The personality is (afaik) some kind of"] # [doc = " value used for C++ unwinding, which must filter by type: we"] # [doc = " don't really care about it very much. Anyway, this value"] # [doc = " contains an alloca into which the personality is stored and"] # [doc = " then later loaded when generating the DIVERGE_BLOCK."] personality_slot : Option < PlaceRef < 'tcx , Bx :: Value > > , # [doc = " A backend `BasicBlock` for each MIR `BasicBlock`, created lazily"] # [doc = " as-needed (e.g. RPO reaching it or another block branching to it)."] cached_llbbs : IndexVec < mir :: BasicBlock , CachedLlbb < Bx :: BasicBlock > > , # [doc = " The funclet status of each basic block"] cleanup_kinds : Option < IndexVec < mir :: BasicBlock , analyze :: CleanupKind > > , # [doc = " When targeting MSVC, this stores the cleanup info for each funclet BB."] # [doc = " This is initialized at the same time as the `landing_pads` entry for the"] # [doc = " funclets' head block, i.e. when needed by an unwind / `cleanup_ret` edge."] funclets : IndexVec < mir :: BasicBlock , Option < Bx :: Funclet > > , # [doc = " This stores the cached landing/cleanup pad block for a given BB."] landing_pads : IndexVec < mir :: BasicBlock , Option < Bx :: BasicBlock > > , # [doc = " Cached unreachable block"] unreachable_block : Option < Bx :: BasicBlock > , # [doc = " Cached terminate upon unwinding block and its reason"] terminate_block : Option < (Bx :: BasicBlock , UnwindTerminateReason) > , # [doc = " A bool flag for each basic block indicating whether it is a cold block."] # [doc = " A cold block is a block that is unlikely to be executed at runtime."] cold_blocks : IndexVec < mir :: BasicBlock , bool > , # [doc = " The location where each MIR arg/var/tmp/ret is stored. This is"] # [doc = " usually an `PlaceRef` representing an alloca, but not always:"] # [doc = " sometimes we can skip the alloca and just store the value"] # [doc = " directly using an `OperandRef`, which makes for tighter LLVM"] # [doc = " IR. The conditions for using an `OperandRef` are as follows:"] # [doc = ""] # [doc = " - the type of the local must be judged \"immediate\" by `is_llvm_immediate`"] # [doc = " - the operand must never be referenced indirectly"] # [doc = "     - we should not take its address using the `&` operator"] # [doc = "     - nor should it appear in a place path like `tmp.a`"] # [doc = " - the operand must be defined by an rvalue that can generate immediate"] # [doc = "   values"] # [doc = ""] # [doc = " Avoiding allocs can also be important for certain intrinsics,"] # [doc = " notably `expect`."] locals : locals :: Locals < 'tcx , Bx :: Value > , # [doc = " All `VarDebugInfo` from the MIR body, partitioned by `Local`."] # [doc = " This is `None` if no variable debuginfo/names are needed."] per_local_var_debug_info : Option < PerLocalVarDebugInfoIndexVec < 'tcx , Bx :: DIVariable > > , # [doc = " Caller location propagated if this function has `#[track_caller]`."] caller_location : Option < OperandRef < 'tcx , Bx :: Value > > , }}}
mkitem!{mkimpl!{impl < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > FunctionCx < 'a , 'tcx , Bx > { pub fn monomorphize < T > (& self , value : T) -> T where T : Copy + TypeFoldable < TyCtxt < 'tcx > > , { debug ! ("monomorphize: self.instance={:?}" , self . instance) ; self . instance . instantiate_mir_and_normalize_erasing_regions (self . cx . tcx () , self . cx . typing_env () , ty :: EarlyBinder :: bind (value) ,) } }}}
mkitem!{mkenum!{enum LocalRef < 'tcx , V > { Place (PlaceRef < 'tcx , V >) , # [doc = " `UnsizedPlace(p)`: `p` itself is a thin pointer (indirect place)."] # [doc = " `*p` is the wide pointer that references the actual unsized place."] # [doc = ""] # [doc = " MIR only supports unsized args, not dynamically-sized locals, so"] # [doc = " new unsized temps don't exist and we must reuse the referred-to place."] # [doc = ""] # [doc = " FIXME: Since the removal of unsized locals in <https://github.com/rust-lang/rust/pull/142911>,"] # [doc = " can we maybe use `Place` here? Or refactor it in another way? There are quite a few"] # [doc = " `UnsizedPlace => bug` branches now."] UnsizedPlace (PlaceRef < 'tcx , V >) , # [doc = " The backend [`OperandValue`] has already been generated."] Operand (OperandRef < 'tcx , V >) , # [doc = " Will be a `Self::Operand` once we get to its definition."] PendingOperand , }}}
mkitem!{mkimpl!{impl < 'tcx , V : CodegenObject > LocalRef < 'tcx , V > { fn new_operand (layout : TyAndLayout < 'tcx >) -> LocalRef < 'tcx , V > { if layout . is_zst () { LocalRef :: Operand (OperandRef :: zero_sized (layout)) } else { LocalRef :: PendingOperand } } }}}

macro_rules! codegen_mir_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_mir in module {}", module_path!());
    };
}

mkfn!{
    codegen_mir_introspect!();
    # [instrument (level = "debug" , skip (cx))] pub fn codegen_mir < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (cx : & 'a Bx :: CodegenCx , instance : Instance < 'tcx > ,) { assert ! (! instance . args . has_infer ()) ; let tcx = cx . tcx () ; let llfn = cx . get_fn (instance) ; let mut mir = tcx . instance_mir (instance . def) ; let fn_abi = cx . fn_abi_of_instance (instance , ty :: List :: empty ()) ; debug ! ("fn_abi: {:?}" , fn_abi) ; if tcx . features () . ergonomic_clones () { let monomorphized_mir = instance . instantiate_mir_and_normalize_erasing_regions (tcx , ty :: TypingEnv :: fully_monomorphized () , ty :: EarlyBinder :: bind (mir . clone ()) ,) ; mir = tcx . arena . alloc (optimize_use_clone :: < Bx > (cx , monomorphized_mir)) ; } let debug_context = cx . create_function_debug_context (instance , fn_abi , llfn , & mir) ; let start_llbb = Bx :: append_block (cx , llfn , "start") ; let mut start_bx = Bx :: build (cx , start_llbb) ; if mir . basic_blocks . iter () . any (| bb | { bb . is_cleanup || matches ! (bb . terminator () . unwind () , Some (mir :: UnwindAction :: Terminate (_))) }) { start_bx . set_personality_fn (cx . eh_personality ()) ; } let cleanup_kinds = base :: wants_new_eh_instructions (tcx . sess) . then (| | analyze :: cleanup_kinds (& mir)) ; let cached_llbbs : IndexVec < mir :: BasicBlock , CachedLlbb < Bx :: BasicBlock > > = mir . basic_blocks . indices () . map (| bb | { if bb == mir :: START_BLOCK { CachedLlbb :: Some (start_llbb) } else { CachedLlbb :: None } }) . collect () ; let mut fx = FunctionCx { instance , mir , llfn , fn_abi , cx , personality_slot : None , cached_llbbs , unreachable_block : None , terminate_block : None , cleanup_kinds , landing_pads : IndexVec :: from_elem (None , & mir . basic_blocks) , funclets : IndexVec :: from_fn_n (| _ | None , mir . basic_blocks . len ()) , cold_blocks : find_cold_blocks (tcx , mir) , locals : locals :: Locals :: empty () , debug_context , per_local_var_debug_info : None , caller_location : None , } ; let (per_local_var_debug_info , consts_debug_info) = fx . compute_per_local_var_debug_info (& mut start_bx) . unzip () ; fx . per_local_var_debug_info = per_local_var_debug_info ; let traversal_order = traversal :: mono_reachable_reverse_postorder (mir , tcx , instance) ; let memory_locals = analyze :: non_ssa_locals (& fx , & traversal_order) ; let local_values = { let args = arg_local_refs (& mut start_bx , & mut fx , & memory_locals) ; let mut allocate_local = | local : Local | { let decl = & mir . local_decls [local] ; let layout = start_bx . layout_of (fx . monomorphize (decl . ty)) ; assert ! (! layout . ty . has_erasable_regions ()) ; if local == mir :: RETURN_PLACE { match fx . fn_abi . ret . mode { PassMode :: Indirect { .. } => { debug ! ("alloc: {:?} (return place) -> place" , local) ; let llretptr = start_bx . get_param (0) ; return LocalRef :: Place (PlaceRef :: new_sized (llretptr , layout)) ; } PassMode :: Cast { ref cast , .. } => { debug ! ("alloc: {:?} (return place) -> place" , local) ; let size = cast . size (& start_bx) . max (layout . size) ; return LocalRef :: Place (PlaceRef :: alloca_size (& mut start_bx , size , layout)) ; } _ => { } } ; } if memory_locals . contains (local) { debug ! ("alloc: {:?} -> place" , local) ; if layout . is_unsized () { LocalRef :: UnsizedPlace (PlaceRef :: alloca_unsized_indirect (& mut start_bx , layout)) } else { LocalRef :: Place (PlaceRef :: alloca (& mut start_bx , layout)) } } else { debug ! ("alloc: {:?} -> operand" , local) ; LocalRef :: new_operand (layout) } } ; let retptr = allocate_local (mir :: RETURN_PLACE) ; iter :: once (retptr) . chain (args . into_iter ()) . chain (mir . vars_and_temps_iter () . map (allocate_local)) . collect () } ; fx . initialize_locals (local_values) ; fx . debug_introduce_locals (& mut start_bx , consts_debug_info . unwrap_or_default ()) ; drop (start_bx) ; let mut unreached_blocks = DenseBitSet :: new_filled (mir . basic_blocks . len ()) ; for bb in traversal_order { fx . codegen_block (bb) ; unreached_blocks . remove (bb) ; } for bb in unreached_blocks . iter () { fx . codegen_block_as_unreachable (bb) ; } }
}

macro_rules! optimize_use_clone_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function optimize_use_clone in module {}", module_path!());
    };
}

mkfn!{
    optimize_use_clone_introspect!();
    fn optimize_use_clone < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (cx : & 'a Bx :: CodegenCx , mut mir : Body < 'tcx > ,) -> Body < 'tcx > { let tcx = cx . tcx () ; if tcx . features () . ergonomic_clones () { for bb in mir . basic_blocks . as_mut () { let mir :: TerminatorKind :: Call { args , destination , target , call_source : mir :: CallSource :: Use , .. } = & bb . terminator () . kind else { continue ; } ; assert_eq ! (args . len () , 1) ; let arg = & args [0] ; let arg_ty = arg . node . ty (& mir . local_decls , tcx) ; let ty :: Ref (_region , inner_ty , mir :: Mutability :: Not) = * arg_ty . kind () else { continue } ; if ! tcx . type_is_copy_modulo_regions (cx . typing_env () , inner_ty) { continue ; } let Some (arg_place) = arg . node . place () else { continue } ; let destination_block = target . unwrap () ; bb . statements . push (mir :: Statement :: new (bb . terminator () . source_info , mir :: StatementKind :: Assign (Box :: new ((* destination , mir :: Rvalue :: Use (mir :: Operand :: Copy (arg_place . project_deeper (& [mir :: ProjectionElem :: Deref] , tcx) ,)) ,))) ,)) ; bb . terminator_mut () . kind = mir :: TerminatorKind :: Goto { target : destination_block } ; } } mir }
}

macro_rules! arg_local_refs_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function arg_local_refs in module {}", module_path!());
    };
}

mkfn!{
    arg_local_refs_introspect!();
    # [doc = " Produces, for each argument, a `Value` pointing at the"] # [doc = " argument's value. As arguments are places, these are always"] # [doc = " indirect."] fn arg_local_refs < 'a , 'tcx , Bx : BuilderMethods < 'a , 'tcx > > (bx : & mut Bx , fx : & mut FunctionCx < 'a , 'tcx , Bx > , memory_locals : & DenseBitSet < mir :: Local > ,) -> Vec < LocalRef < 'tcx , Bx :: Value > > { let mir = fx . mir ; let mut idx = 0 ; let mut llarg_idx = fx . fn_abi . ret . is_indirect () as usize ; let mut num_untupled = None ; let codegen_fn_attrs = bx . tcx () . codegen_instance_attrs (fx . instance . def) ; if codegen_fn_attrs . flags . contains (CodegenFnAttrFlags :: NAKED) { return vec ! [] ; } let args = mir . args_iter () . enumerate () . map (| (arg_index , local) | { let arg_decl = & mir . local_decls [local] ; let arg_ty = fx . monomorphize (arg_decl . ty) ; if Some (local) == mir . spread_arg { let ty :: Tuple (tupled_arg_tys) = arg_ty . kind () else { bug ! ("spread argument isn't a tuple?!") ; } ; let layout = bx . layout_of (arg_ty) ; if layout . is_unsized () { span_bug ! (arg_decl . source_info . span , "\"rust-call\" ABI does not support unsized params" ,) ; } let place = PlaceRef :: alloca (bx , layout) ; for i in 0 .. tupled_arg_tys . len () { let arg = & fx . fn_abi . args [idx] ; idx += 1 ; if let PassMode :: Cast { pad_i32 : true , .. } = arg . mode { llarg_idx += 1 ; } let pr_field = place . project_field (bx , i) ; bx . store_fn_arg (arg , & mut llarg_idx , pr_field) ; } assert_eq ! (None , num_untupled . replace (tupled_arg_tys . len ()) , "Replaced existing num_tupled") ; return LocalRef :: Place (place) ; } if fx . fn_abi . c_variadic && arg_index == fx . fn_abi . args . len () { let va_list = PlaceRef :: alloca (bx , bx . layout_of (arg_ty)) ; bx . lifetime_start (va_list . val . llval , va_list . layout . size) ; bx . va_start (va_list . val . llval) ; return LocalRef :: Place (va_list) ; } let arg = & fx . fn_abi . args [idx] ; idx += 1 ; if let PassMode :: Cast { pad_i32 : true , .. } = arg . mode { llarg_idx += 1 ; } if ! memory_locals . contains (local) { let local = | op | LocalRef :: Operand (op) ; match arg . mode { PassMode :: Ignore => { return local (OperandRef :: zero_sized (arg . layout)) ; } PassMode :: Direct (_) => { let llarg = bx . get_param (llarg_idx) ; llarg_idx += 1 ; return local (OperandRef :: from_immediate_or_packed_pair (bx , llarg , arg . layout ,)) ; } PassMode :: Pair (..) => { let (a , b) = (bx . get_param (llarg_idx) , bx . get_param (llarg_idx + 1)) ; llarg_idx += 2 ; return local (OperandRef { val : OperandValue :: Pair (a , b) , layout : arg . layout , }) ; } _ => { } } } match arg . mode { PassMode :: Indirect { attrs , meta_attrs : None , on_stack : _ } => { if let Some (pointee_align) = attrs . pointee_align && pointee_align < arg . layout . align . abi { let tmp = PlaceRef :: alloca (bx , arg . layout) ; bx . store_fn_arg (arg , & mut llarg_idx , tmp) ; LocalRef :: Place (tmp) } else { let llarg = bx . get_param (llarg_idx) ; llarg_idx += 1 ; LocalRef :: Place (PlaceRef :: new_sized (llarg , arg . layout)) } } PassMode :: Indirect { attrs : _ , meta_attrs : Some (_) , on_stack : _ } => { let llarg = bx . get_param (llarg_idx) ; llarg_idx += 1 ; let llextra = bx . get_param (llarg_idx) ; llarg_idx += 1 ; let indirect_operand = OperandValue :: Pair (llarg , llextra) ; let tmp = PlaceRef :: alloca_unsized_indirect (bx , arg . layout) ; indirect_operand . store (bx , tmp) ; LocalRef :: UnsizedPlace (tmp) } _ => { let tmp = PlaceRef :: alloca (bx , arg . layout) ; bx . store_fn_arg (arg , & mut llarg_idx , tmp) ; LocalRef :: Place (tmp) } } }) . collect :: < Vec < _ > > () ; if fx . instance . def . requires_caller_location (bx . tcx ()) { let mir_args = if let Some (num_untupled) = num_untupled { args . len () - 1 + num_untupled } else { args . len () } ; assert_eq ! (fx . fn_abi . args . len () , mir_args + 1 , "#[track_caller] instance {:?} must have 1 more argument in their ABI than in their MIR" , fx . instance) ; let arg = fx . fn_abi . args . last () . unwrap () ; match arg . mode { PassMode :: Direct (_) => () , _ => bug ! ("caller location must be PassMode::Direct, found {:?}" , arg . mode) , } fx . caller_location = Some (OperandRef { val : OperandValue :: Immediate (bx . get_param (llarg_idx)) , layout : arg . layout , }) ; } args }
}

macro_rules! find_cold_blocks_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function find_cold_blocks in module {}", module_path!());
    };
}

mkfn!{
    find_cold_blocks_introspect!();
    fn find_cold_blocks < 'tcx > (tcx : TyCtxt < 'tcx > , mir : & mir :: Body < 'tcx > ,) -> IndexVec < mir :: BasicBlock , bool > { let local_decls = & mir . local_decls ; let mut cold_blocks : IndexVec < mir :: BasicBlock , bool > = IndexVec :: from_elem (false , & mir . basic_blocks) ; for (bb , bb_data) in traversal :: postorder (mir) { let terminator = bb_data . terminator () ; match terminator . kind { mir :: TerminatorKind :: Call { ref func , .. } | mir :: TerminatorKind :: TailCall { ref func , .. } if let ty :: FnDef (def_id , ..) = * func . ty (local_decls , tcx) . kind () && let attrs = tcx . codegen_fn_attrs (def_id) && attrs . flags . contains (CodegenFnAttrFlags :: COLD) => { cold_blocks [bb] = true ; continue ; } mir :: TerminatorKind :: Unreachable => { cold_blocks [bb] = true ; continue ; } _ => { } } let mut succ = terminator . successors () ; if let Some (first) = succ . next () && cold_blocks [first] && succ . all (| s | cold_blocks [s]) { cold_blocks [bb] = true ; } } cold_blocks }
}