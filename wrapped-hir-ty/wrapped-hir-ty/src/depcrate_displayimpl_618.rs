// Generated macro for impl_618 (impl)
macro_rules! Depcrate_displayimpl_618 {
() => {
// Module: crate::display
// Provides: {"impl_618"}
// Dependencies: {}
impl HirDisplay for CallableSig { fn hir_fmt (& self , f : & mut HirFormatter < '_ >) -> Result < () , HirDisplayError > { let CallableSig { params_and_return : _ , is_varargs , safety , abi : _ } = * self ; if let Safety :: Unsafe = safety { write ! (f , "unsafe ") ? ; } write ! (f , "fn(") ? ; f . write_joined (self . params () , ", ") ? ; if is_varargs { if self . params () . is_empty () { write ! (f , "...") ? ; } else { write ! (f , ", ...") ? ; } } write ! (f , ")") ? ; let ret = self . ret () ; if ! ret . is_unit () { write ! (f , " -> ") ? ; ret . hir_fmt (f) ? ; } Ok (()) } }
};
}
