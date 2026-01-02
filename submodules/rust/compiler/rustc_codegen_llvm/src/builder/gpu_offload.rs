mkuse!{use std :: ffi :: CString ;}
mkuse!{use llvm :: Linkage :: * ;}
mkuse!{use rustc_abi :: Align ;}
mkuse!{use rustc_codegen_ssa :: back :: write :: CodegenContext ;}
mkuse!{use rustc_codegen_ssa :: traits :: BaseTypeCodegenMethods ;}
mkuse!{use crate :: builder :: SBuilder ;}
mkuse!{use crate :: common :: AsCCharPtr ;}
mkuse!{use crate :: llvm :: AttributePlace :: Function ;}
mkuse!{use crate :: llvm :: { self , Linkage , Type , Value } ;}
mkuse!{use crate :: { LlvmCodegenBackend , SimpleCx , attributes } ;}

macro_rules! handle_gpu_code_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function handle_gpu_code in module {}", module_path!());
    };
}

mkfn!{
    handle_gpu_code_introspect!();
    pub (crate) fn handle_gpu_code < 'll > (_cgcx : & CodegenContext < LlvmCodegenBackend > , cx : & 'll SimpleCx < '_ > ,) { let mut o_types = vec ! [] ; let mut kernels = vec ! [] ; let offload_entry_ty = add_tgt_offload_entry (& cx) ; for num in 0 .. 9 { let kernel = cx . get_function (& format ! ("kernel_{num}")) ; if let Some (kernel) = kernel { o_types . push (gen_define_handling (& cx , kernel , offload_entry_ty , num)) ; kernels . push (kernel) ; } } gen_call_handling (& cx , & kernels , & o_types) ; }
}

macro_rules! generate_at_one_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function generate_at_one in module {}", module_path!());
    };
}

mkfn!{
    generate_at_one_introspect!();
    fn generate_at_one < 'll > (cx : & 'll SimpleCx < '_ >) -> & 'll llvm :: Value { let unknown_txt = ";unknown;unknown;0;0;;" ; let c_entry_name = CString :: new (unknown_txt) . unwrap () ; let c_val = c_entry_name . as_bytes_with_nul () ; let initializer = crate :: common :: bytes_in_context (cx . llcx , c_val) ; let at_zero = add_unnamed_global (& cx , & "" , initializer , PrivateLinkage) ; llvm :: set_alignment (at_zero , Align :: ONE) ; let struct_ident_ty = cx . type_named_struct ("struct.ident_t") ; let struct_elems = vec ! [cx . get_const_i32 (0) , cx . get_const_i32 (2) , cx . get_const_i32 (0) , cx . get_const_i32 (22) , at_zero ,] ; let struct_elems_ty : Vec < _ > = struct_elems . iter () . map (| & x | cx . val_ty (x)) . collect () ; let initializer = crate :: common :: named_struct (struct_ident_ty , & struct_elems) ; cx . set_struct_body (struct_ident_ty , & struct_elems_ty , false) ; let at_one = add_unnamed_global (& cx , & "" , initializer , PrivateLinkage) ; llvm :: set_alignment (at_one , Align :: EIGHT) ; at_one }
}

macro_rules! add_tgt_offload_entry_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_tgt_offload_entry in module {}", module_path!());
    };
}

mkfn!{
    add_tgt_offload_entry_introspect!();
    pub (crate) fn add_tgt_offload_entry < 'll > (cx : & 'll SimpleCx < '_ >) -> & 'll llvm :: Type { let offload_entry_ty = cx . type_named_struct ("struct.__tgt_offload_entry") ; let tptr = cx . type_ptr () ; let ti64 = cx . type_i64 () ; let ti32 = cx . type_i32 () ; let ti16 = cx . type_i16 () ; let entry_elements = vec ! [ti64 , ti16 , ti16 , ti32 , tptr , tptr , ti64 , ti64 , tptr] ; cx . set_struct_body (offload_entry_ty , & entry_elements , false) ; offload_entry_ty }
}

macro_rules! gen_tgt_kernel_global_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_tgt_kernel_global in module {}", module_path!());
    };
}

mkfn!{
    gen_tgt_kernel_global_introspect!();
    fn gen_tgt_kernel_global < 'll > (cx : & 'll SimpleCx < '_ >) { let kernel_arguments_ty = cx . type_named_struct ("struct.__tgt_kernel_arguments") ; let tptr = cx . type_ptr () ; let ti64 = cx . type_i64 () ; let ti32 = cx . type_i32 () ; let tarr = cx . type_array (ti32 , 3) ; let kernel_elements = vec ! [ti32 , ti32 , tptr , tptr , tptr , tptr , tptr , tptr , ti64 , ti64 , tarr , tarr , ti32] ; cx . set_struct_body (kernel_arguments_ty , & kernel_elements , false) ; cx . declare_global ("my_struct_global2" , kernel_arguments_ty) ; }
}

macro_rules! gen_tgt_data_mappers_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_tgt_data_mappers in module {}", module_path!());
    };
}

mkfn!{
    gen_tgt_data_mappers_introspect!();
    fn gen_tgt_data_mappers < 'll > (cx : & 'll SimpleCx < '_ > ,) -> (& 'll llvm :: Value , & 'll llvm :: Value , & 'll llvm :: Value , & 'll llvm :: Type) { let tptr = cx . type_ptr () ; let ti64 = cx . type_i64 () ; let ti32 = cx . type_i32 () ; let args = vec ! [tptr , ti64 , ti32 , tptr , tptr , tptr , tptr , tptr , tptr] ; let mapper_fn_ty = cx . type_func (& args , cx . type_void ()) ; let mapper_begin = "__tgt_target_data_begin_mapper" ; let mapper_update = "__tgt_target_data_update_mapper" ; let mapper_end = "__tgt_target_data_end_mapper" ; let begin_mapper_decl = declare_offload_fn (& cx , mapper_begin , mapper_fn_ty) ; let update_mapper_decl = declare_offload_fn (& cx , mapper_update , mapper_fn_ty) ; let end_mapper_decl = declare_offload_fn (& cx , mapper_end , mapper_fn_ty) ; let nounwind = llvm :: AttributeKind :: NoUnwind . create_attr (cx . llcx) ; attributes :: apply_to_llfn (begin_mapper_decl , Function , & [nounwind]) ; attributes :: apply_to_llfn (update_mapper_decl , Function , & [nounwind]) ; attributes :: apply_to_llfn (end_mapper_decl , Function , & [nounwind]) ; (begin_mapper_decl , update_mapper_decl , end_mapper_decl , mapper_fn_ty) }
}

macro_rules! add_priv_unnamed_arr_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_priv_unnamed_arr in module {}", module_path!());
    };
}

mkfn!{
    add_priv_unnamed_arr_introspect!();
    fn add_priv_unnamed_arr < 'll > (cx : & SimpleCx < 'll > , name : & str , vals : & [u64]) -> & 'll llvm :: Value { let ti64 = cx . type_i64 () ; let mut size_val = Vec :: with_capacity (vals . len ()) ; for & val in vals { size_val . push (cx . get_const_i64 (val)) ; } let initializer = cx . const_array (ti64 , & size_val) ; add_unnamed_global (cx , name , initializer , PrivateLinkage) }
}

macro_rules! add_unnamed_global_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_unnamed_global in module {}", module_path!());
    };
}

mkfn!{
    add_unnamed_global_introspect!();
    pub (crate) fn add_unnamed_global < 'll > (cx : & SimpleCx < 'll > , name : & str , initializer : & 'll llvm :: Value , l : Linkage ,) -> & 'll llvm :: Value { let llglobal = add_global (cx , name , initializer , l) ; llvm :: LLVMSetUnnamedAddress (llglobal , llvm :: UnnamedAddr :: Global) ; llglobal }
}

macro_rules! add_global_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function add_global in module {}", module_path!());
    };
}

mkfn!{
    add_global_introspect!();
    pub (crate) fn add_global < 'll > (cx : & SimpleCx < 'll > , name : & str , initializer : & 'll llvm :: Value , l : Linkage ,) -> & 'll llvm :: Value { let c_name = CString :: new (name) . unwrap () ; let llglobal : & 'll llvm :: Value = llvm :: add_global (cx . llmod , cx . val_ty (initializer) , & c_name) ; llvm :: set_global_constant (llglobal , true) ; llvm :: set_linkage (llglobal , l) ; llvm :: set_initializer (llglobal , initializer) ; llglobal }
}

macro_rules! gen_define_handling_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_define_handling in module {}", module_path!());
    };
}

mkfn!{
    gen_define_handling_introspect!();
    fn gen_define_handling < 'll > (cx : & 'll SimpleCx < '_ > , kernel : & 'll llvm :: Value , offload_entry_ty : & 'll llvm :: Type , num : i64 ,) -> & 'll llvm :: Value { let types = cx . func_params_types (cx . get_type_of_global (kernel)) ; let num_ptr_types = types . iter () . filter (| & x | matches ! (cx . type_kind (x) , rustc_codegen_ssa :: common :: TypeKind :: Pointer)) . count () ; add_priv_unnamed_arr (& cx , & format ! (".offload_sizes.{num}") , & vec ! [1024 ; num_ptr_types]) ; let o_types = add_priv_unnamed_arr (& cx , & format ! (".offload_maptypes.{num}") , & vec ! [3 ; num_ptr_types]) ; let name = format ! (".kernel_{num}.region_id") ; let initializer = cx . get_const_i8 (0) ; let region_id = add_unnamed_global (& cx , & name , initializer , WeakAnyLinkage) ; let c_entry_name = CString :: new (format ! ("kernel_{num}")) . unwrap () ; let c_val = c_entry_name . as_bytes_with_nul () ; let offload_entry_name = format ! (".offloading.entry_name.{num}") ; let initializer = crate :: common :: bytes_in_context (cx . llcx , c_val) ; let llglobal = add_unnamed_global (& cx , & offload_entry_name , initializer , InternalLinkage) ; llvm :: set_alignment (llglobal , Align :: ONE) ; llvm :: set_section (llglobal , c".llvm.rodata.offloading") ; let name = format ! (".offloading.entry.kernel_{num}") ; let reserved = cx . get_const_i64 (0) ; let version = cx . get_const_i16 (1) ; let kind = cx . get_const_i16 (1) ; let flags = cx . get_const_i32 (0) ; let size = cx . get_const_i64 (0) ; let data = cx . get_const_i64 (0) ; let aux_addr = cx . const_null (cx . type_ptr ()) ; let elems = vec ! [reserved , version , kind , flags , region_id , llglobal , size , data , aux_addr] ; let initializer = crate :: common :: named_struct (offload_entry_ty , & elems) ; let c_name = CString :: new (name) . unwrap () ; let llglobal = llvm :: add_global (cx . llmod , offload_entry_ty , & c_name) ; llvm :: set_global_constant (llglobal , true) ; llvm :: set_linkage (llglobal , WeakAnyLinkage) ; llvm :: set_initializer (llglobal , initializer) ; llvm :: set_alignment (llglobal , Align :: ONE) ; let c_section_name = CString :: new (".omp_offloading_entries") . unwrap () ; llvm :: set_section (llglobal , & c_section_name) ; o_types }
}

macro_rules! declare_offload_fn_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function declare_offload_fn in module {}", module_path!());
    };
}

mkfn!{
    declare_offload_fn_introspect!();
    fn declare_offload_fn < 'll > (cx : & 'll SimpleCx < '_ > , name : & str , ty : & 'll llvm :: Type ,) -> & 'll llvm :: Value { crate :: declare :: declare_simple_fn (cx , name , llvm :: CallConv :: CCallConv , llvm :: UnnamedAddr :: No , llvm :: Visibility :: Default , ty ,) }
}

macro_rules! gen_call_handling_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function gen_call_handling in module {}", module_path!());
    };
}

mkfn!{
    gen_call_handling_introspect!();
    fn gen_call_handling < 'll > (cx : & 'll SimpleCx < '_ > , _kernels : & [& 'll llvm :: Value] , o_types : & [& 'll llvm :: Value] ,) { let tptr = cx . type_ptr () ; let ti32 = cx . type_i32 () ; let tgt_bin_desc_ty = vec ! [ti32 , tptr , tptr , tptr] ; let tgt_bin_desc = cx . type_named_struct ("struct.__tgt_bin_desc") ; cx . set_struct_body (tgt_bin_desc , & tgt_bin_desc_ty , false) ; gen_tgt_kernel_global (& cx) ; let (begin_mapper_decl , _ , end_mapper_decl , fn_ty) = gen_tgt_data_mappers (& cx) ; let main_fn = cx . get_function ("main") ; let Some (main_fn) = main_fn else { return } ; let kernel_name = "kernel_1" ; let call = unsafe { llvm :: LLVMRustGetFunctionCall (main_fn , kernel_name . as_c_char_ptr () , kernel_name . len ()) } ; let Some (kernel_call) = call else { return ; } ; let kernel_call_bb = unsafe { llvm :: LLVMGetInstructionParent (kernel_call) } ; let called = unsafe { llvm :: LLVMGetCalledValue (kernel_call) . unwrap () } ; let mut builder = SBuilder :: build (cx , kernel_call_bb) ; let types = cx . func_params_types (cx . get_type_of_global (called)) ; let num_args = types . len () as u64 ; unsafe { llvm :: LLVMRustPositionBuilderPastAllocas (builder . llbuilder , main_fn) } ; let tgt_bin_desc_alloca = builder . direct_alloca (tgt_bin_desc , Align :: EIGHT , "EmptyDesc") ; let ty = cx . type_array (cx . type_ptr () , num_args) ; let a1 = builder . direct_alloca (ty , Align :: EIGHT , ".offload_baseptrs") ; let a2 = builder . direct_alloca (ty , Align :: EIGHT , ".offload_ptrs") ; let ty2 = cx . type_array (cx . type_i64 () , num_args) ; let a4 = builder . direct_alloca (ty2 , Align :: EIGHT , ".offload_sizes") ; let mut vals = vec ! [] ; let mut geps = vec ! [] ; let i32_0 = cx . get_const_i32 (0) ; for (index , in_ty) in types . iter () . enumerate () { let p = llvm :: get_param (called , index as u32) ; let name = llvm :: get_value_name (p) ; let name = str :: from_utf8 (& name) . unwrap () ; let arg_name = format ! ("{name}.addr") ; let alloca = builder . direct_alloca (in_ty , Align :: EIGHT , & arg_name) ; builder . store (p , alloca , Align :: EIGHT) ; let val = builder . load (in_ty , alloca , Align :: EIGHT) ; let gep = builder . inbounds_gep (cx . type_f32 () , val , & [i32_0]) ; vals . push (val) ; geps . push (gep) ; } unsafe { llvm :: LLVMRustPositionBefore (builder . llbuilder , kernel_call) } ; builder . memset (tgt_bin_desc_alloca , cx . get_const_i8 (0) , cx . get_const_i64 (32) , Align :: EIGHT) ; let mapper_fn_ty = cx . type_func (& [cx . type_ptr ()] , cx . type_void ()) ; let register_lib_decl = declare_offload_fn (& cx , "__tgt_register_lib" , mapper_fn_ty) ; let unregister_lib_decl = declare_offload_fn (& cx , "__tgt_unregister_lib" , mapper_fn_ty) ; let init_ty = cx . type_func (& [] , cx . type_void ()) ; let init_rtls_decl = declare_offload_fn (cx , "__tgt_init_all_rtls" , init_ty) ; builder . call (mapper_fn_ty , register_lib_decl , & [tgt_bin_desc_alloca] , None) ; builder . call (init_ty , init_rtls_decl , & [] , None) ; for i in 0 .. num_args { let idx = cx . get_const_i32 (i) ; let gep1 = builder . inbounds_gep (ty , a1 , & [i32_0 , idx]) ; builder . store (vals [i as usize] , gep1 , Align :: EIGHT) ; let gep2 = builder . inbounds_gep (ty , a2 , & [i32_0 , idx]) ; builder . store (geps [i as usize] , gep2 , Align :: EIGHT) ; let gep3 = builder . inbounds_gep (ty2 , a4 , & [i32_0 , idx]) ; builder . store (cx . get_const_i64 (1024) , gep3 , Align :: EIGHT) ; } fn get_geps < 'a , 'll > (builder : & mut SBuilder < 'a , 'll > , cx : & 'll SimpleCx < 'll > , ty : & 'll Type , ty2 : & 'll Type , a1 : & 'll Value , a2 : & 'll Value , a4 : & 'll Value ,) -> (& 'll Value , & 'll Value , & 'll Value) { let i32_0 = cx . get_const_i32 (0) ; let gep1 = builder . inbounds_gep (ty , a1 , & [i32_0 , i32_0]) ; let gep2 = builder . inbounds_gep (ty , a2 , & [i32_0 , i32_0]) ; let gep3 = builder . inbounds_gep (ty2 , a4 , & [i32_0 , i32_0]) ; (gep1 , gep2 , gep3) } fn generate_mapper_call < 'a , 'll > (builder : & mut SBuilder < 'a , 'll > , cx : & 'll SimpleCx < 'll > , geps : (& 'll Value , & 'll Value , & 'll Value) , o_type : & 'll Value , fn_to_call : & 'll Value , fn_ty : & 'll Type , num_args : u64 , s_ident_t : & 'll Value ,) { let nullptr = cx . const_null (cx . type_ptr ()) ; let i64_max = cx . get_const_i64 (u64 :: MAX) ; let num_args = cx . get_const_i32 (num_args) ; let args = vec ! [s_ident_t , i64_max , num_args , geps . 0 , geps . 1 , geps . 2 , o_type , nullptr , nullptr] ; builder . call (fn_ty , fn_to_call , & args , None) ; } let s_ident_t = generate_at_one (& cx) ; let o = o_types [0] ; let geps = get_geps (& mut builder , & cx , ty , ty2 , a1 , a2 , a4) ; generate_mapper_call (& mut builder , & cx , geps , o , begin_mapper_decl , fn_ty , num_args , s_ident_t) ; unsafe { llvm :: LLVMRustPositionAfter (builder . llbuilder , kernel_call) } ; let geps = get_geps (& mut builder , & cx , ty , ty2 , a1 , a2 , a4) ; generate_mapper_call (& mut builder , & cx , geps , o , end_mapper_decl , fn_ty , num_args , s_ident_t) ; builder . call (mapper_fn_ty , unregister_lib_decl , & [tgt_bin_desc_alloca] , None) ; }
}