mkuse!{# [cfg (feature = "master")] use gccjit :: FnAttribute ;}
mkuse!{use gccjit :: { Context , FunctionType , RValue , ToRValue , Type } ;}
mkuse!{use rustc_ast :: expand :: allocator :: { ALLOCATOR_METHODS , AllocatorKind , AllocatorTy , NO_ALLOC_SHIM_IS_UNSTABLE , alloc_error_handler_name , default_fn_name , global_fn_name , } ;}
mkuse!{use rustc_middle :: bug ;}
mkuse!{use rustc_middle :: ty :: TyCtxt ;}
mkuse!{use rustc_session :: config :: OomStrategy ;}
mkuse!{use rustc_symbol_mangling :: mangle_internal_symbol ;}
mkuse!{use crate :: GccContext ;}
mkuse!{# [cfg (feature = "master")] use crate :: base :: symbol_visibility_to_gcc ;}

macro_rules! codegen_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function codegen in module {}", module_path!());
    };
}

mkfn!{
    codegen_introspect!();
    pub (crate) unsafe fn codegen (tcx : TyCtxt < '_ > , mods : & mut GccContext , _module_name : & str , kind : AllocatorKind , alloc_error_handler_kind : AllocatorKind ,) { let context = & mods . context ; let usize = match tcx . sess . target . pointer_width { 16 => context . new_type :: < u16 > () , 32 => context . new_type :: < u32 > () , 64 => context . new_type :: < u64 > () , tws => bug ! ("Unsupported target word size for int: {}" , tws) , } ; let i8 = context . new_type :: < i8 > () ; let i8p = i8 . make_pointer () ; if kind == AllocatorKind :: Default { for method in ALLOCATOR_METHODS { let mut types = Vec :: with_capacity (method . inputs . len ()) ; for input in method . inputs . iter () { match input . ty { AllocatorTy :: Layout => { types . push (usize) ; types . push (usize) ; } AllocatorTy :: Ptr => types . push (i8p) , AllocatorTy :: Usize => types . push (usize) , AllocatorTy :: ResultPtr | AllocatorTy :: Unit => panic ! ("invalid allocator arg") , } } let output = match method . output { AllocatorTy :: ResultPtr => Some (i8p) , AllocatorTy :: Unit => None , AllocatorTy :: Layout | AllocatorTy :: Usize | AllocatorTy :: Ptr => { panic ! ("invalid allocator output") } } ; let from_name = mangle_internal_symbol (tcx , & global_fn_name (method . name)) ; let to_name = mangle_internal_symbol (tcx , & default_fn_name (method . name)) ; create_wrapper_function (tcx , context , & from_name , Some (& to_name) , & types , output) ; } } create_wrapper_function (tcx , context , & mangle_internal_symbol (tcx , "__rust_alloc_error_handler") , Some (& mangle_internal_symbol (tcx , alloc_error_handler_name (alloc_error_handler_kind))) , & [usize , usize] , None ,) ; create_const_value_function (tcx , context , & mangle_internal_symbol (tcx , OomStrategy :: SYMBOL) , i8 , context . new_rvalue_from_int (i8 , tcx . sess . opts . unstable_opts . oom . should_panic () as i32) ,) ; create_wrapper_function (tcx , context , & mangle_internal_symbol (tcx , NO_ALLOC_SHIM_IS_UNSTABLE) , None , & [] , None ,) ; }
}

macro_rules! create_const_value_function_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_const_value_function in module {}", module_path!());
    };
}

mkfn!{
    create_const_value_function_introspect!();
    fn create_const_value_function (tcx : TyCtxt < '_ > , context : & Context < '_ > , name : & str , output : Type < '_ > , value : RValue < '_ > ,) { let func = context . new_function (None , FunctionType :: Exported , output , & [] , name , false) ; # [cfg (feature = "master")] { func . add_attribute (FnAttribute :: Visibility (symbol_visibility_to_gcc (tcx . sess . default_visibility () ,))) ; func . add_attribute (FnAttribute :: Inline) ; } if tcx . sess . must_emit_unwind_tables () { } let block = func . new_block ("entry") ; block . end_with_return (None , value) ; }
}

macro_rules! create_wrapper_function_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function create_wrapper_function in module {}", module_path!());
    };
}

mkfn!{
    create_wrapper_function_introspect!();
    fn create_wrapper_function (tcx : TyCtxt < '_ > , context : & Context < '_ > , from_name : & str , to_name : Option < & str > , types : & [Type < '_ >] , output : Option < Type < '_ > > ,) { let void = context . new_type :: < () > () ; let args : Vec < _ > = types . iter () . enumerate () . map (| (index , typ) | context . new_parameter (None , * typ , format ! ("param{}" , index))) . collect () ; let func = context . new_function (None , FunctionType :: Exported , output . unwrap_or (void) , & args , from_name , false ,) ; # [cfg (feature = "master")] func . add_attribute (FnAttribute :: Visibility (symbol_visibility_to_gcc (tcx . sess . default_visibility () ,))) ; if tcx . sess . must_emit_unwind_tables () { } let block = func . new_block ("entry") ; if let Some (to_name) = to_name { let args : Vec < _ > = types . iter () . enumerate () . map (| (index , typ) | context . new_parameter (None , * typ , format ! ("param{}" , index))) . collect () ; let callee = context . new_function (None , FunctionType :: Extern , output . unwrap_or (void) , & args , to_name , false ,) ; # [cfg (feature = "master")] callee . add_attribute (FnAttribute :: Visibility (gccjit :: Visibility :: Hidden)) ; let args = args . iter () . enumerate () . map (| (i , _) | func . get_param (i as i32) . to_rvalue ()) . collect :: < Vec < _ > > () ; let ret = context . new_call (None , callee , & args) ; if output . is_some () { block . end_with_return (None , ret) ; } else { block . add_eval (None , ret) ; block . end_with_void_return (None) ; } } else { assert ! (output . is_none ()) ; block . end_with_void_return (None) ; } }
}