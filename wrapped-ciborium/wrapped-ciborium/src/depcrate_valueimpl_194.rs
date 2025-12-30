// Generated macro for impl_194 (impl)
macro_rules! Depcrate_valueimpl_194 {
() => {
// Module: crate::value
// Provides: {"impl_194"}
// Dependencies: {}
impl From < u128 > for Value { # [inline] fn from (value : u128) -> Self { if let Ok (x) = Integer :: try_from (value) { return Value :: Integer (x) ; } let mut bytes = & value . to_be_bytes () [..] ; while let Some (0) = bytes . first () { bytes = & bytes [1 ..] ; } Value :: Tag (ciborium_ll :: tag :: BIGPOS , Value :: Bytes (bytes . into ()) . into ()) } }
};
}
