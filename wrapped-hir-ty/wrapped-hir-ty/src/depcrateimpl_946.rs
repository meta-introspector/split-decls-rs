// Generated macro for impl_946 (impl)
macro_rules! Depcrateimpl_946 {
() => {
// Module: crate
// Provides: {"impl_946"}
// Dependencies: {}
impl TypeFoldable < Interner > for CallableSig { fn try_fold_with < E > (self , folder : & mut dyn chalk_ir :: fold :: FallibleTypeFolder < Interner , Error = E > , outer_binder : DebruijnIndex ,) -> Result < Self , E > { let vec = self . params_and_return . to_vec () ; let folded = vec . try_fold_with (folder , outer_binder) ? ; Ok (CallableSig { params_and_return : folded . into () , is_varargs : self . is_varargs , safety : self . safety , abi : self . abi , }) } }
};
}
