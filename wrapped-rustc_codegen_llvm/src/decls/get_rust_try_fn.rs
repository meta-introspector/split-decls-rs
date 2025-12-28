macro_rules! deps {
    () => {
        Builder!();
        CodegenCx!();
    };
}

macro_rules! get_rust_try_fn {
    () => {
        deps!();
        fn get_rust_try_fn < 'a , 'll , 'tcx > (cx : & 'a CodegenCx < 'll , 'tcx > , codegen : & mut dyn FnMut (Builder < 'a , 'll , 'tcx >) ,) -> (& 'll Type , & 'll Value) { if let Some (llfn) = cx . rust_try_fn . get () { return llfn ; } let tcx = cx . tcx ; let i8p = Ty :: new_mut_ptr (tcx , tcx . types . i8) ; let try_fn_ty = Ty :: new_fn_ptr (tcx , ty :: Binder :: dummy (tcx . mk_fn_sig ([i8p] , tcx . types . unit , false , hir :: Safety :: Unsafe , ExternAbi :: Rust ,)) ,) ; let catch_fn_ty = Ty :: new_fn_ptr (tcx , ty :: Binder :: dummy (tcx . mk_fn_sig ([i8p , i8p] , tcx . types . unit , false , hir :: Safety :: Unsafe , ExternAbi :: Rust ,)) ,) ; let rust_fn_sig = ty :: Binder :: dummy (cx . tcx . mk_fn_sig ([try_fn_ty , i8p , catch_fn_ty] , tcx . types . i32 , false , hir :: Safety :: Unsafe , ExternAbi :: Rust ,)) ; let rust_try = gen_fn (cx , "__rust_try" , rust_fn_sig , codegen) ; cx . rust_try_fn . set (Some (rust_try)) ; rust_try }
    };
}

get_rust_try_fn!();