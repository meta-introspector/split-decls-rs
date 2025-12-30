// Generated macro for raw_codec (macro)
macro_rules! Depcrate_rawraw_codec {
() => {
// Module: crate::raw
// Provides: {"raw_codec"}
// Dependencies: {}
# [macro_export] macro_rules ! raw_codec { ($ ($ fn : ident => [$ ($ chunk : expr ,) +] ;) *) => { { let mut b = $ crate :: prelude :: mock_io :: Builder :: new () ; $ ({ let mut chunk = vec ! [] ; $ ($ crate :: raw :: Chunk :: push (&$ chunk , & mut chunk) ;) + b .$ fn (& chunk [..]) ; }) * $ crate :: Codec :: new (b . build ()) } } }
};
}
