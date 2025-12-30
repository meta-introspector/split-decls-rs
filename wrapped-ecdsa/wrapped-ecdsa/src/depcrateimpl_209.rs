// Generated macro for impl_209 (impl)
macro_rules! Depcrateimpl_209 {
() => {
// Module: crate
// Provides: {"impl_209"}
// Dependencies: {}
# [cfg (feature = "algorithm")] impl < C > str :: FromStr for Signature < C > where C : EcdsaCurve + CurveArithmetic , SignatureSize < C > : ArraySize , { type Err = Error ; fn from_str (hex : & str) -> Result < Self > { if hex . len () != C :: FieldBytesSize :: USIZE * 4 { return Err (Error :: new ()) ; } if ! hex . as_bytes () . iter () . all (| & byte | matches ! (byte , b'0' ..= b'9' | b'a' ..= b'z' | b'A' ..= b'Z')) { return Err (Error :: new ()) ; } let (r_hex , s_hex) = hex . split_at (C :: FieldBytesSize :: USIZE * 2) ; let r = r_hex . parse :: < NonZeroScalar < C > > () . map_err (| _ | Error :: new ()) ? ; let s = s_hex . parse :: < NonZeroScalar < C > > () . map_err (| _ | Error :: new ()) ? ; Self :: from_scalars (r , s) } }
};
}
