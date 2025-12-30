// Generated macro for impl_195 (impl)
macro_rules! Depcrate_valueimpl_195 {
() => {
// Module: crate::value
// Provides: {"impl_195"}
// Dependencies: {}
impl From < i128 > for Value { # [inline] fn from (value : i128) -> Self { if let Ok (x) = Integer :: try_from (value) { return Value :: Integer (x) ; } let (tag , raw) = match value . is_negative () { true => (ciborium_ll :: tag :: BIGNEG , value as u128 ^ ! 0) , false => (ciborium_ll :: tag :: BIGPOS , value as u128) , } ; let mut bytes = & raw . to_be_bytes () [..] ; while let Some (0) = bytes . first () { bytes = & bytes [1 ..] ; } Value :: Tag (tag , Value :: Bytes (bytes . into ()) . into ()) } }
};
}
