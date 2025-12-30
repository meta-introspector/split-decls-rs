// Generated macro for wrap_data_funcs_for_compatibility (macro)
macro_rules! Depcrate_decodewrap_data_funcs_for_compatibility {
() => {
// Module: crate::decode
// Provides: {"wrap_data_funcs_for_compatibility"}
// Dependencies: {}
macro_rules ! wrap_data_funcs_for_compatibility { ($ ($ tp : ident) ,* $ (,) ?) => { $ (paste :: paste ! { # [cfg (feature = "std")] # [doc (hidden)] # [deprecated (note = "internal function. rmpv & rmp-serde need to switch to RmpRead")] pub fn [< read_data_ $ tp >] < R : std :: io :: Read > (buf : & mut R) -> Result <$ tp , ValueReadError > { buf . [< read_data_ $ tp >] () } }) * } ; }
};
}
