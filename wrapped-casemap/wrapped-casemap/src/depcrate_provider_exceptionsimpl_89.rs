// Generated macro for impl_89 (impl)
macro_rules! Depcrate_provider_exceptionsimpl_89 {
() => {
// Module: crate::provider::exceptions
// Provides: {"impl_89"}
// Dependencies: {}
impl CaseMapExceptions < '_ > { # [doc = " Obtain the exception at index `idx`. Will"] # [doc = " return a default value if not present (GIGO behavior),"] # [doc = " as these indices should come from a paired CaseMapData object"] # [doc = ""] # [doc = " Will also panic in debug mode"] pub fn get (& self , idx : u16) -> & ExceptionULE { let exception = self . exceptions . get (idx . into ()) ; debug_assert ! (exception . is_some ()) ; exception . unwrap_or (ExceptionULE :: empty_exception ()) } # [cfg (any (feature = "serde" , feature = "datagen"))] pub (crate) fn validate (& self) -> Result < Range < u16 > , & 'static str > { for exception in self . exceptions . iter () { exception . validate () ? ; } u16 :: try_from (self . exceptions . len ()) . map_err (| _ | "Too many exceptions") . map (| l | 0 .. l) } }
};
}
