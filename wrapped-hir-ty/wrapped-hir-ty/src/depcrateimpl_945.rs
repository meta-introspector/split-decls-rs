// Generated macro for impl_945 (impl)
macro_rules! Depcrateimpl_945 {
() => {
// Module: crate
// Provides: {"impl_945"}
// Dependencies: {}
impl CallableSig { pub fn from_params_and_return (params : impl Iterator < Item = Ty > , ret : Ty , is_varargs : bool , safety : Safety , abi : FnAbi ,) -> CallableSig { let mut params_and_return = Vec :: with_capacity (params . size_hint () . 0 + 1) ; params_and_return . extend (params) ; params_and_return . push (ret) ; CallableSig { params_and_return : params_and_return . into () , is_varargs , safety , abi } } pub fn from_def (db : & dyn HirDatabase , def : FnDefId , substs : & Substitution) -> CallableSig { let callable_def = ToChalk :: from_chalk (db , def) ; let sig = db . callable_item_signature (callable_def) ; sig . substitute (Interner , substs) } pub fn from_fn_ptr (fn_ptr : & FnPointer) -> CallableSig { CallableSig { params_and_return : Arc :: from_iter (fn_ptr . substitution . clone () . shifted_out_to (Interner , DebruijnIndex :: ONE) . expect ("unexpected lifetime vars in fn ptr") . 0 . as_slice (Interner) . iter () . map (| arg | arg . assert_ty_ref (Interner) . clone ()) ,) , is_varargs : fn_ptr . sig . variadic , safety : fn_ptr . sig . safety , abi : fn_ptr . sig . abi , } } pub fn to_fn_ptr (& self) -> FnPointer { FnPointer { num_binders : 0 , sig : FnSig { abi : self . abi , safety : self . safety , variadic : self . is_varargs } , substitution : FnSubst (Substitution :: from_iter (Interner , self . params_and_return . iter () . cloned () ,)) , } } pub fn abi (& self) -> FnAbi { self . abi } pub fn params (& self) -> & [Ty] { & self . params_and_return [0 .. self . params_and_return . len () - 1] } pub fn ret (& self) -> & Ty { & self . params_and_return [self . params_and_return . len () - 1] } }
};
}
