// Generated macro for impl_215 (impl)
macro_rules! Depcrate_hirimpl_215 {
() => {
// Module: crate::hir
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'hir > ConstArg < 'hir > { # [doc = " Converts a `ConstArg` in an unambiguous position to one in an ambiguous position. This is"] # [doc = " fallible as the [`ConstArgKind::Infer`] variant is not present in ambiguous positions."] # [doc = ""] # [doc = " Functions accepting ambiguous consts will not handle the [`ConstArgKind::Infer`] variant, if"] # [doc = " infer consts are relevant to you then care should be taken to handle them separately."] pub fn try_as_ambig_ct (& self) -> Option < & ConstArg < 'hir , AmbigArg > > { if let ConstArgKind :: Infer (_ , ()) = self . kind { return None ; } let ptr = self as * const ConstArg < 'hir > as * const ConstArg < 'hir , AmbigArg > ; Some (unsafe { & * ptr }) } }
};
}
