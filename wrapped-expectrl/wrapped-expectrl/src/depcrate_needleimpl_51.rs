// Generated macro for impl_51 (impl)
macro_rules! Depcrate_needleimpl_51 {
() => {
// Module: crate::needle
// Provides: {"impl_51"}
// Dependencies: {}
impl < Re : AsRef < str > > Needle for Regex < Re > { fn check (& self , buf : & [u8] , _ : bool) -> Result < Vec < Match > , Error > { let regex = regex :: bytes :: Regex :: new (self . 0 . as_ref ()) . map_err (| _ | Error :: RegexParsing) ? ; let matches = regex . captures_iter (buf) . flat_map (| c | c . iter () . flatten () . map (| m | m . into ()) . collect :: < Vec < Match > > ()) . collect () ; Ok (matches) } }
};
}
