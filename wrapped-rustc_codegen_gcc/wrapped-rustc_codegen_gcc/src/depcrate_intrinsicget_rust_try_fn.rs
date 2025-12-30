// Generated macro for get_rust_try_fn (function)
macro_rules! Depcrate_intrinsicget_rust_try_fn {
() => {
// Module: crate::intrinsic
// Provides: {"get_rust_try_fn"}
// Dependencies: {}
# [cfg (feature = "master")] fn get_rust_try_fn < 'a , 'gcc , 'tcx > (cx : & 'a CodegenCx < 'gcc , 'tcx > , codegen : & mut dyn FnMut (Builder < 'a , 'gcc , 'tcx >) ,) -> (Type < 'gcc > , Function < 'gcc >) { if let Some (llfn) = cx . rust_try_fn . get () { return llfn ; } let tcx = cx . tcx ; let i8p = Ty :: new_mut_ptr (tcx , tcx . types . i8) ; let try_fn_ty = Ty :: new_fn_ptr (tcx , ty :: Binder :: dummy (tcx . mk_fn_sig (iter :: once (i8p) , tcx . types . unit , false , rustc_hir :: Safety :: Unsafe , ExternAbi :: Rust ,)) ,) ; let catch_fn_ty = Ty :: new_fn_ptr (tcx , ty :: Binder :: dummy (tcx . mk_fn_sig ([i8p , i8p] . iter () . cloned () , tcx . types . unit , false , rustc_hir :: Safety :: Unsafe , ExternAbi :: Rust ,)) ,) ; let rust_fn_sig = ty :: Binder :: dummy (cx . tcx . mk_fn_sig ([try_fn_ty , i8p , catch_fn_ty] , tcx . types . i32 , false , rustc_hir :: Safety :: Unsafe , ExternAbi :: Rust ,)) ; let rust_try = gen_fn (cx , "__rust_try" , rust_fn_sig , codegen) ; cx . rust_try_fn . set (Some (rust_try)) ; rust_try }
};
}
