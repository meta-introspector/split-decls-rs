mkmod!{comments, { 
                getname!(comments);
                getsrc!(comments);
                getpath!(comments);
                get_deps!(comments);
                get_crates!(comments);
                mkinclude!(comments);
                 
            }}
mkmod!{pass_mode, { 
                getname!(pass_mode);
                getsrc!(pass_mode);
                getpath!(pass_mode);
                get_deps!(pass_mode);
                get_crates!(pass_mode);
                mkinclude!(pass_mode);
                 
            }}
mkmod!{returning, { 
                getname!(returning);
                getsrc!(returning);
                getpath!(returning);
                get_deps!(returning);
                get_crates!(returning);
                mkinclude!(returning);
                 
            }}
mkuse!{use std :: borrow :: Cow ;}
mkuse!{use std :: mem ;}
mkuse!{use cranelift_codegen :: ir :: { ArgumentPurpose , SigRef } ;}
mkuse!{use cranelift_codegen :: isa :: CallConv ;}
mkuse!{use cranelift_module :: ModuleError ;}
mkuse!{use rustc_abi :: { CanonAbi , ExternAbi , X86Call } ;}
mkuse!{use rustc_codegen_ssa :: base :: is_call_from_compiler_builtins_to_upstream_monomorphization ;}
mkuse!{use rustc_codegen_ssa :: errors :: CompilerBuiltinsCannotCall ;}
mkuse!{use rustc_middle :: middle :: codegen_fn_attrs :: CodegenFnAttrFlags ;}
mkuse!{use rustc_middle :: ty :: TypeVisitableExt ;}
mkuse!{use rustc_middle :: ty :: layout :: FnAbiOf ;}
mkuse!{use rustc_middle :: ty :: print :: with_no_trimmed_paths ;}
mkuse!{use rustc_session :: Session ;}
mkuse!{use rustc_span :: source_map :: Spanned ;}
mkuse!{use rustc_target :: callconv :: { FnAbi , PassMode } ;}
mkuse!{use smallvec :: SmallVec ;}
mkuse!{use self :: pass_mode :: * ;}
mkuse!{pub (crate) use self :: returning :: codegen_return ;}
mkuse!{use crate :: prelude :: * ;}

macro_rules! clif_sig_from_fn_abi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function clif_sig_from_fn_abi in module {}", module_path!());
    };
}

mkfn!{
    clif_sig_from_fn_abi_introspect!();
    fn clif_sig_from_fn_abi < 'tcx > (tcx : TyCtxt < 'tcx > , default_call_conv : CallConv , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > ,) -> Signature { let call_conv = conv_to_call_conv (tcx . sess , fn_abi . conv , default_call_conv) ; let inputs = fn_abi . args . iter () . flat_map (| arg_abi | arg_abi . get_abi_param (tcx) . into_iter ()) ; let (return_ptr , returns) = fn_abi . ret . get_abi_return (tcx) ; let params : Vec < _ > = return_ptr . into_iter () . chain (inputs) . collect () ; Signature { params , returns , call_conv } }
}

macro_rules! conv_to_call_conv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function conv_to_call_conv in module {}", module_path!());
    };
}

mkfn!{
    conv_to_call_conv_introspect!();
    pub (crate) fn conv_to_call_conv (sess : & Session , c : CanonAbi , default_call_conv : CallConv ,) -> CallConv { match c { CanonAbi :: Rust | CanonAbi :: C => default_call_conv , CanonAbi :: RustCold => CallConv :: Cold , CanonAbi :: Custom => default_call_conv , CanonAbi :: X86 (x86_call) => match x86_call { X86Call :: SysV64 => CallConv :: SystemV , X86Call :: Win64 => CallConv :: WindowsFastcall , _ => default_call_conv , } , CanonAbi :: Interrupt (_) | CanonAbi :: Arm (_) => { sess . dcx () . fatal ("call conv {c:?} is not yet implemented") } CanonAbi :: GpuKernel => { unreachable ! ("tried to use {c:?} call conv which only exists on an unsupported target") } } }
}

macro_rules! get_function_sig_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function get_function_sig in module {}", module_path!());
    };
}

mkfn!{
    get_function_sig_introspect!();
    pub (crate) fn get_function_sig < 'tcx > (tcx : TyCtxt < 'tcx > , default_call_conv : CallConv , inst : Instance < 'tcx > ,) -> Signature { assert ! (! inst . args . has_infer ()) ; clif_sig_from_fn_abi (tcx , default_call_conv , & FullyMonomorphizedLayoutCx (tcx) . fn_abi_of_instance (inst , ty :: List :: empty ()) ,) }
}

macro_rules! import_function_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function import_function in module {}", module_path!());
    };
}

mkfn!{
    import_function_introspect!();
    # [doc = " Instance must be monomorphized"] pub (crate) fn import_function < 'tcx > (tcx : TyCtxt < 'tcx > , module : & mut dyn Module , inst : Instance < 'tcx > ,) -> FuncId { let name = tcx . symbol_name (inst) . name ; let sig = get_function_sig (tcx , module . target_config () . default_call_conv , inst) ; match module . declare_function (name , Linkage :: Import , & sig) { Ok (func_id) => func_id , Err (ModuleError :: IncompatibleDeclaration (_)) => tcx . dcx () . fatal (format ! ("attempt to declare `{name}` as function, but it was already declared as static")) , Err (ModuleError :: IncompatibleSignature (_ , prev_sig , new_sig)) => tcx . dcx () . fatal (format ! ("attempt to declare `{name}` with signature {new_sig:?}, \
             but it was already declared with signature {prev_sig:?}")) , Err (err) => Err :: < _ , _ > (err) . unwrap () , } }
}
mkitem!{mkimpl!{impl < 'tcx > FunctionCx < '_ , '_ , 'tcx > { # [doc = " Instance must be monomorphized"] pub (crate) fn get_function_ref (& mut self , inst : Instance < 'tcx >) -> FuncRef { let func_id = import_function (self . tcx , self . module , inst) ; let func_ref = self . module . declare_func_in_func (func_id , & mut self . bcx . func) ; if self . clif_comments . enabled () { self . add_comment (func_ref , format ! ("{:?}" , inst)) ; } func_ref } pub (crate) fn lib_call (& mut self , name : & str , params : Vec < AbiParam > , mut returns : Vec < AbiParam > , args : & [Value] ,) -> Cow < '_ , [Value] > { let (params , args) : (Vec < _ > , Cow < '_ , [_] >) = if self . tcx . sess . target . is_like_windows { let (params , args) : (Vec < _ > , Vec < _ >) = params . into_iter () . zip (args) . map (| (param , & arg) | { if param . value_type == types :: I128 { let arg_ptr = self . create_stack_slot (16 , 16) ; arg_ptr . store (self , arg , MemFlags :: trusted ()) ; (AbiParam :: new (self . pointer_type) , arg_ptr . get_addr (self)) } else { (param , arg) } }) . unzip () ; (params , args . into ()) } else { (params , args . into ()) } ; let ret_single_i128 = returns . len () == 1 && returns [0] . value_type == types :: I128 ; if ret_single_i128 && self . tcx . sess . target . is_like_windows { returns [0] . value_type = types :: I64X2 ; let ret = self . lib_call_unadjusted (name , params , returns , & args) [0] ; Cow :: Owned (vec ! [codegen_bitcast (self , types :: I128 , ret)]) } else if ret_single_i128 && self . tcx . sess . target . arch == "s390x" { let mut params = params ; let mut args = args . to_vec () ; params . insert (0 , AbiParam :: new (self . pointer_type)) ; let ret_ptr = self . create_stack_slot (16 , 16) ; args . insert (0 , ret_ptr . get_addr (self)) ; self . lib_call_unadjusted (name , params , vec ! [] , & args) ; Cow :: Owned (vec ! [ret_ptr . load (self , types :: I128 , MemFlags :: trusted ())]) } else { Cow :: Borrowed (self . lib_call_unadjusted (name , params , returns , & args)) } } fn lib_call_unadjusted (& mut self , name : & str , params : Vec < AbiParam > , returns : Vec < AbiParam > , args : & [Value] ,) -> & [Value] { let sig = Signature { params , returns , call_conv : self . target_config . default_call_conv } ; let func_id = self . module . declare_function (name , Linkage :: Import , & sig) . unwrap () ; let func_ref = self . module . declare_func_in_func (func_id , & mut self . bcx . func) ; let call_inst = self . bcx . ins () . call (func_ref , args) ; if self . clif_comments . enabled () { self . add_comment (func_ref , format ! ("{:?}" , name)) ; self . add_comment (call_inst , format ! ("lib_call {}" , name)) ; } let results = self . bcx . inst_results (call_inst) ; assert ! (results . len () <= 2 , "{}" , results . len ()) ; results } }}}

macro_rules! make_local_place_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function make_local_place in module {}", module_path!());
    };
}

mkfn!{
    make_local_place_introspect!();
    # [doc = " Make a [`CPlace`] capable of holding value of the specified type."] fn make_local_place < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , local : Local , layout : TyAndLayout < 'tcx > , is_ssa : bool ,) -> CPlace < 'tcx > { if layout . is_unsized () { fx . tcx . dcx () . span_fatal (fx . mir . local_decls [local] . source_info . span , "unsized locals are not yet supported" ,) ; } let place = if is_ssa { if let BackendRepr :: ScalarPair (_ , _) = layout . backend_repr { CPlace :: new_var_pair (fx , local , layout) } else { CPlace :: new_var (fx , local , layout) } } else { CPlace :: new_stack_slot (fx , layout) } ; self :: comments :: add_local_place_comments (fx , place , local) ; place }
}

macro_rules! codegen_fn_prelude_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_fn_prelude in module {}", module_path!());
    };
}

mkfn!{
    codegen_fn_prelude_introspect!();
    pub (crate) fn codegen_fn_prelude < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , start_block : Block) { fx . bcx . append_block_params_for_function_params (start_block) ; fx . bcx . switch_to_block (start_block) ; fx . bcx . ins () . nop () ; let ssa_analyzed = crate :: analyze :: analyze (fx) ; self :: comments :: add_args_header_comment (fx) ; let mut block_params_iter = fx . bcx . func . dfg . block_params (start_block) . to_vec () . into_iter () ; let ret_place = self :: returning :: codegen_return_param (fx , & ssa_analyzed , & mut block_params_iter) ; assert_eq ! (fx . local_map . push (ret_place) , RETURN_PLACE) ; enum ArgKind < 'tcx > { Normal (Option < CValue < 'tcx > >) , Spread (Vec < Option < CValue < 'tcx > > >) , } if fx . fn_abi . c_variadic { fx . tcx . dcx () . span_fatal (fx . mir . span , "Defining variadic functions is not yet supported by Cranelift" ,) ; } let mut arg_abis_iter = fx . fn_abi . args . iter () ; let func_params = fx . mir . args_iter () . map (| local | { let arg_ty = fx . monomorphize (fx . mir . local_decls [local] . ty) ; if Some (local) == fx . mir . spread_arg { let tupled_arg_tys = match arg_ty . kind () { ty :: Tuple (ref tys) => tys , _ => bug ! ("spread argument isn't a tuple?! but {:?}" , arg_ty) , } ; let mut params = Vec :: new () ; for (i , _arg_ty) in tupled_arg_tys . iter () . enumerate () { let arg_abi = arg_abis_iter . next () . unwrap () ; let param = cvalue_for_param (fx , Some (local) , Some (i) , arg_abi , & mut block_params_iter) ; params . push (param) ; } (local , ArgKind :: Spread (params) , arg_ty) } else { let arg_abi = arg_abis_iter . next () . unwrap () ; let param = cvalue_for_param (fx , Some (local) , None , arg_abi , & mut block_params_iter) ; (local , ArgKind :: Normal (param) , arg_ty) } }) . collect :: < Vec < (Local , ArgKind < 'tcx > , Ty < 'tcx >) > > () ; assert ! (fx . caller_location . is_none ()) ; if fx . instance . def . requires_caller_location (fx . tcx) { let arg_abi = arg_abis_iter . next () . unwrap () ; fx . caller_location = Some (cvalue_for_param (fx , None , None , arg_abi , & mut block_params_iter) . unwrap ()) ; } assert ! (arg_abis_iter . next () . is_none () , "ArgAbi left behind") ; assert ! (block_params_iter . next () . is_none () , "arg_value left behind") ; self :: comments :: add_locals_header_comment (fx) ; for (local , arg_kind , ty) in func_params { if let ArgKind :: Normal (Some (val)) = arg_kind { if let Some ((addr , meta)) = val . try_to_ptr () { let place = if let Some (meta) = meta { CPlace :: for_ptr_with_extra (addr , meta , val . layout ()) } else { CPlace :: for_ptr (addr , val . layout ()) } ; self :: comments :: add_local_place_comments (fx , place , local) ; assert_eq ! (fx . local_map . push (place) , local) ; continue ; } } let layout = fx . layout_of (ty) ; let is_ssa = ssa_analyzed [local] . is_ssa (fx , ty) ; let place = make_local_place (fx , local , layout , is_ssa) ; assert_eq ! (fx . local_map . push (place) , local) ; match arg_kind { ArgKind :: Normal (param) => { if let Some (param) = param { place . write_cvalue (fx , param) ; } } ArgKind :: Spread (params) => { for (i , param) in params . into_iter () . enumerate () { if let Some (param) = param { place . place_field (fx , FieldIdx :: new (i)) . write_cvalue (fx , param) ; } } } } } for local in fx . mir . vars_and_temps_iter () { let ty = fx . monomorphize (fx . mir . local_decls [local] . ty) ; let layout = fx . layout_of (ty) ; let is_ssa = ssa_analyzed [local] . is_ssa (fx , ty) ; let place = make_local_place (fx , local , layout , is_ssa) ; assert_eq ! (fx . local_map . push (place) , local) ; } fx . bcx . ins () . jump (* fx . block_map . get (START_BLOCK) . unwrap () , & []) ; }
}
mkitem!{mkstruct!{struct CallArgument < 'tcx > { value : CValue < 'tcx > , is_owned : bool , }}}

macro_rules! codegen_call_argument_operand_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_call_argument_operand in module {}", module_path!());
    };
}

mkfn!{
    codegen_call_argument_operand_introspect!();
    fn codegen_call_argument_operand < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , operand : & Operand < 'tcx > ,) -> CallArgument < 'tcx > { CallArgument { value : codegen_operand (fx , operand) , is_owned : matches ! (operand , Operand :: Move (_)) , } }
}

macro_rules! codegen_terminator_call_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_terminator_call in module {}", module_path!());
    };
}

mkfn!{
    codegen_terminator_call_introspect!();
    pub (crate) fn codegen_terminator_call < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , source_info : mir :: SourceInfo , func : & Operand < 'tcx > , args : & [Spanned < Operand < 'tcx > >] , destination : Place < 'tcx > , target : Option < BasicBlock > , _unwind : UnwindAction ,) { let func = codegen_operand (fx , func) ; let fn_sig = func . layout () . ty . fn_sig (fx . tcx) ; let ret_place = codegen_place (fx , destination) ; let instance = if let ty :: FnDef (def_id , fn_args) = * func . layout () . ty . kind () { let instance = ty :: Instance :: expect_resolve (fx . tcx , ty :: TypingEnv :: fully_monomorphized () , def_id , fn_args , source_info . span ,) ; if is_call_from_compiler_builtins_to_upstream_monomorphization (fx . tcx , instance) { if target . is_some () { let caller_def = fx . instance . def_id () ; let e = CompilerBuiltinsCannotCall { span : fx . tcx . def_span (caller_def) , caller : with_no_trimmed_paths ! (fx . tcx . def_path_str (caller_def)) , callee : with_no_trimmed_paths ! (fx . tcx . def_path_str (def_id)) , } ; fx . tcx . dcx () . emit_err (e) ; } else { fx . bcx . ins () . trap (TrapCode :: user (2) . unwrap ()) ; return ; } } if fx . tcx . symbol_name (instance) . name . starts_with ("llvm.") { crate :: intrinsics :: codegen_llvm_intrinsic_call (fx , & fx . tcx . symbol_name (instance) . name , args , ret_place , target , source_info . span ,) ; return ; } match instance . def { InstanceKind :: Intrinsic (_) => { match crate :: intrinsics :: codegen_intrinsic_call (fx , instance , args , ret_place , target , source_info ,) { Ok (()) => return , Err (instance) => Some (instance) , } } InstanceKind :: DropGlue (_ , None) => { let dest = target . expect ("Non terminating drop_in_place_real???") ; let ret_block = fx . get_block (dest) ; fx . bcx . ins () . jump (ret_block , & []) ; return ; } _ => Some (instance) , } } else { None } ; let extra_args = & args [fn_sig . inputs () . skip_binder () . len () ..] ; let extra_args = fx . tcx . mk_type_list_from_iter (extra_args . iter () . map (| op_arg | fx . monomorphize (op_arg . node . ty (fx . mir , fx . tcx))) ,) ; let fn_abi = if let Some (instance) = instance { FullyMonomorphizedLayoutCx (fx . tcx) . fn_abi_of_instance (instance , extra_args) } else { FullyMonomorphizedLayoutCx (fx . tcx) . fn_abi_of_fn_ptr (fn_sig , extra_args) } ; let is_cold = if fn_sig . abi () == ExternAbi :: RustCold { true } else { instance . is_some_and (| inst | { fx . tcx . codegen_fn_attrs (inst . def_id ()) . flags . contains (CodegenFnAttrFlags :: COLD) }) } ; if is_cold { fx . bcx . set_cold_block (fx . bcx . current_block () . unwrap ()) ; if let Some (destination_block) = target { fx . bcx . set_cold_block (fx . get_block (destination_block)) ; } } let mut args = if fn_sig . abi () == ExternAbi :: RustCall { let (self_arg , pack_arg) = match args { [pack_arg] => (None , codegen_call_argument_operand (fx , & pack_arg . node)) , [self_arg , pack_arg] => (Some (codegen_call_argument_operand (fx , & self_arg . node)) , codegen_call_argument_operand (fx , & pack_arg . node) ,) , _ => panic ! ("rust-call abi requires one or two arguments") , } ; let tupled_arguments = match pack_arg . value . layout () . ty . kind () { ty :: Tuple (ref tupled_arguments) => tupled_arguments , _ => bug ! ("argument to function with \"rust-call\" ABI is not a tuple") , } ; let mut args = Vec :: with_capacity (1 + tupled_arguments . len ()) ; args . extend (self_arg) ; for i in 0 .. tupled_arguments . len () { args . push (CallArgument { value : pack_arg . value . value_field (fx , FieldIdx :: new (i)) , is_owned : pack_arg . is_owned , }) ; } args } else { args . iter () . map (| arg | codegen_call_argument_operand (fx , & arg . node)) . collect :: < Vec < _ > > () } ; if instance . is_some_and (| inst | inst . def . requires_caller_location (fx . tcx)) { let caller_location = fx . get_caller_location (source_info) ; args . push (CallArgument { value : caller_location , is_owned : false }) ; } let args = args ; assert_eq ! (fn_abi . args . len () , args . len ()) ; # [derive (Copy , Clone)] enum CallTarget { Direct (FuncRef) , Indirect (SigRef , Value) , } let (func_ref , first_arg_override) = match instance { Some (Instance { def : InstanceKind :: Virtual (_ , idx) , .. }) => { if fx . clif_comments . enabled () { let nop_inst = fx . bcx . ins () . nop () ; fx . add_post_comment (nop_inst , with_no_trimmed_paths ! (format ! ("virtual call; self arg pass mode: {:?}" , fn_abi . args [0])) ,) ; } let (ptr , method) = crate :: vtable :: get_ptr_and_method_ref (fx , args [0] . value , idx) ; let sig = clif_sig_from_fn_abi (fx . tcx , fx . target_config . default_call_conv , & fn_abi) ; let sig = fx . bcx . import_signature (sig) ; (CallTarget :: Indirect (sig , method) , Some (ptr . get_addr (fx))) } Some (instance) => { let func_ref = fx . get_function_ref (instance) ; (CallTarget :: Direct (func_ref) , None) } None => { if fx . clif_comments . enabled () { let nop_inst = fx . bcx . ins () . nop () ; fx . add_post_comment (nop_inst , "indirect call") ; } let func = func . load_scalar (fx) ; let sig = clif_sig_from_fn_abi (fx . tcx , fx . target_config . default_call_conv , & fn_abi) ; let sig = fx . bcx . import_signature (sig) ; (CallTarget :: Indirect (sig , func) , None) } } ; self :: returning :: codegen_with_call_return_arg (fx , & fn_abi . ret , ret_place , | fx , return_ptr | { let mut call_args = return_ptr . into_iter () . chain (first_arg_override . into_iter ()) . chain (args . into_iter () . enumerate () . skip (if first_arg_override . is_some () { 1 } else { 0 }) . flat_map (| (i , arg) | { adjust_arg_for_abi (fx , arg . value , & fn_abi . args [i] , arg . is_owned) . into_iter () }) ,) . collect :: < Vec < Value > > () ; if fn_abi . c_variadic { adjust_call_for_c_variadic (fx , & fn_abi , source_info , func_ref , & mut call_args) ; } let call_inst = match func_ref { CallTarget :: Direct (func_ref) => fx . bcx . ins () . call (func_ref , & call_args) , CallTarget :: Indirect (sig , func_ptr) => { fx . bcx . ins () . call_indirect (sig , func_ptr , & call_args) } } ; if fx . clif_comments . enabled () { with_no_trimmed_paths ! (fx . add_comment (call_inst , format ! ("abi: {:?}" , fn_abi))) ; } fx . bcx . func . dfg . inst_results (call_inst) . iter () . copied () . collect :: < SmallVec < [Value ; 2] > > () }) ; if let Some (dest) = target { let ret_block = fx . get_block (dest) ; fx . bcx . ins () . jump (ret_block , & []) ; } else { fx . bcx . ins () . trap (TrapCode :: user (1) . unwrap ()) ; } fn adjust_call_for_c_variadic < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , fn_abi : & FnAbi < 'tcx , Ty < 'tcx > > , source_info : mir :: SourceInfo , target : CallTarget , call_args : & mut Vec < Value > ,) { if fn_abi . conv != CanonAbi :: C { fx . tcx . dcx () . span_fatal (source_info . span , format ! ("Variadic call for non-C abi {:?}" , fn_abi . conv) ,) ; } let sig_ref = match target { CallTarget :: Direct (func_ref) => fx . bcx . func . dfg . ext_funcs [func_ref] . signature , CallTarget :: Indirect (sig_ref , _) => sig_ref , } ; let mut abi_params = mem :: take (& mut fx . bcx . func . dfg . signatures [sig_ref] . params) ; let has_return_arg = matches ! (fn_abi . ret . mode , PassMode :: Indirect { .. }) ; abi_params . truncate (if has_return_arg { 1 } else { 0 }) ; abi_params . extend (fn_abi . args [.. fn_abi . fixed_count as usize] . iter () . flat_map (| arg_abi | arg_abi . get_abi_param (fx . tcx) . into_iter ()) ,) ; let fixed_arg_count = abi_params . len () ; abi_params . extend (fn_abi . args [fn_abi . fixed_count as usize ..] . iter () . flat_map (| arg_abi | arg_abi . get_abi_param (fx . tcx) . into_iter ()) ,) ; if fx . tcx . sess . target . is_like_darwin && fx . tcx . sess . target . arch == "aarch64" { if abi_params . len () > fixed_arg_count { let integer_registers_used : usize = abi_params [if has_return_arg { 1 } else { 0 } .. fixed_arg_count] . iter () . map (| arg | if arg . value_type . bits () == 128 { 2 } else { 1 }) . sum () ; if integer_registers_used < 8 { abi_params . splice (fixed_arg_count .. fixed_arg_count , (integer_registers_used .. 8) . map (| _ | AbiParam :: new (types :: I64)) ,) ; call_args . splice (fixed_arg_count .. fixed_arg_count , (integer_registers_used .. 8) . map (| _ | fx . bcx . ins () . iconst (types :: I64 , 0)) ,) ; } } assert ! (abi_params . iter () . all (| param | matches ! (param . purpose , ArgumentPurpose :: Normal | ArgumentPurpose :: StructReturn))) ; } for param in abi_params . iter () { if ! param . value_type . is_int () { fx . tcx . dcx () . span_fatal (source_info . span , format ! ("Non int ty {:?} for variadic call" , param . value_type) ,) ; } } assert_eq ! (abi_params . len () , call_args . len ()) ; fx . bcx . func . dfg . signatures [sig_ref] . params = abi_params ; } }
}

macro_rules! codegen_drop_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen_drop in module {}", module_path!());
    };
}

mkfn!{
    codegen_drop_introspect!();
    pub (crate) fn codegen_drop < 'tcx > (fx : & mut FunctionCx < '_ , '_ , 'tcx > , source_info : mir :: SourceInfo , drop_place : CPlace < 'tcx > , target : BasicBlock , _unwind : UnwindAction ,) { let ty = drop_place . layout () . ty ; let drop_instance = Instance :: resolve_drop_in_place (fx . tcx , ty) ; let ret_block = fx . get_block (target) ; if let ty :: InstanceKind :: DropGlue (_ , None) = drop_instance . def { fx . bcx . ins () . jump (ret_block , & []) ; } else { match ty . kind () { ty :: Dynamic (_ , _ , ty :: Dyn) => { let (ptr , vtable) = drop_place . to_ptr_unsized () ; let ptr = ptr . get_addr (fx) ; let drop_fn = crate :: vtable :: drop_fn_of_obj (fx , vtable) ; let is_null = fx . bcx . ins () . icmp_imm (IntCC :: Equal , drop_fn , 0) ; let target_block = fx . get_block (target) ; let continued = fx . bcx . create_block () ; fx . bcx . ins () . brif (is_null , target_block , & [] , continued , & []) ; fx . bcx . switch_to_block (continued) ; let virtual_drop = Instance { def : ty :: InstanceKind :: Virtual (drop_instance . def_id () , 0) , args : drop_instance . args , } ; let fn_abi = FullyMonomorphizedLayoutCx (fx . tcx) . fn_abi_of_instance (virtual_drop , ty :: List :: empty ()) ; let sig = clif_sig_from_fn_abi (fx . tcx , fx . target_config . default_call_conv , & fn_abi) ; let sig = fx . bcx . import_signature (sig) ; fx . bcx . ins () . call_indirect (sig , drop_fn , & [ptr]) ; fx . bcx . ins () . jump (ret_block , & []) ; } _ => { assert ! (! matches ! (drop_instance . def , InstanceKind :: Virtual (_ , _))) ; let fn_abi = FullyMonomorphizedLayoutCx (fx . tcx) . fn_abi_of_instance (drop_instance , ty :: List :: empty ()) ; let arg_value = drop_place . place_ref (fx , fx . layout_of (Ty :: new_mut_ref (fx . tcx , fx . tcx . lifetimes . re_erased , ty)) ,) ; let arg_value = adjust_arg_for_abi (fx , arg_value , & fn_abi . args [0] , true) ; let mut call_args : Vec < Value > = arg_value . into_iter () . collect :: < Vec < _ > > () ; if drop_instance . def . requires_caller_location (fx . tcx) { let caller_location = fx . get_caller_location (source_info) ; call_args . extend (adjust_arg_for_abi (fx , caller_location , & fn_abi . args [1] , false) . into_iter () ,) ; } let func_ref = fx . get_function_ref (drop_instance) ; fx . bcx . ins () . call (func_ref , & call_args) ; fx . bcx . ins () . jump (ret_block , & []) ; } } } }
}

macro_rules! lib_call_arg_param_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function lib_call_arg_param in module {}", module_path!());
    };
}

mkfn!{
    lib_call_arg_param_introspect!();
    pub (crate) fn lib_call_arg_param (tcx : TyCtxt < '_ > , ty : Type , is_signed : bool) -> AbiParam { let param = AbiParam :: new (ty) ; if ty . is_int () && u64 :: from (ty . bits ()) < tcx . data_layout . pointer_size () . bits () { match (& * tcx . sess . target . arch , & * tcx . sess . target . vendor) { ("x86_64" , _) | ("aarch64" , "apple") => match (ty , is_signed) { (types :: I8 | types :: I16 , true) => param . sext () , (types :: I8 | types :: I16 , false) => param . uext () , _ => param , } , ("aarch64" , _) => param , ("riscv64" , _) => match (ty , is_signed) { (types :: I32 , _) | (_ , true) => param . sext () , _ => param . uext () , } , ("s390x" , _) => { if is_signed { param . sext () } else { param . uext () } } _ => unimplemented ! ("{:?}" , tcx . sess . target . arch) , } } else { param } }
}