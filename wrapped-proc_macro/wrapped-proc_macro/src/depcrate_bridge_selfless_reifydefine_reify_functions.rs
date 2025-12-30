// Generated macro for define_reify_functions (macro)
macro_rules! Depcrate_bridge_selfless_reifydefine_reify_functions {
() => {
// Module: crate::bridge::selfless_reify
// Provides: {"define_reify_functions"}
// Dependencies: {}
macro_rules ! define_reify_functions { ($ (fn $ name : ident $ (<$ ($ param : ident) ,*>) ? for $ (extern $ abi : tt) ? fn ($ ($ arg : ident : $ arg_ty : ty) ,*) -> $ ret_ty : ty ;) +) => { $ (pub (super) const fn $ name < $ ($ ($ param ,) *) ? F : Fn ($ ($ arg_ty) ,*) -> $ ret_ty + Copy > (f : F) -> $ (extern $ abi) ? fn ($ ($ arg_ty) ,*) -> $ ret_ty { assert ! (size_of ::< F > () == 0 , "selfless_reify: closure must be zero-sized") ; $ (extern $ abi) ? fn wrapper < $ ($ ($ param ,) *) ? F : Fn ($ ($ arg_ty) ,*) -> $ ret_ty + Copy > ($ ($ arg : $ arg_ty) ,*) -> $ ret_ty { let f = unsafe { mem :: MaybeUninit ::< F >:: uninit () . assume_init () } ; f ($ ($ arg) ,*) } let _f_proof = f ; wrapper ::< $ ($ ($ param ,) *) ? F > }) + } }
};
}
