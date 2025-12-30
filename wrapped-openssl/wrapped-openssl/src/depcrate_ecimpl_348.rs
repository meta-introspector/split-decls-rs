// Generated macro for impl_348 (impl)
macro_rules! Depcrate_ecimpl_348 {
() => {
// Module: crate::ec
// Provides: {"impl_348"}
// Dependencies: {}
impl EcPoint { # [doc = " Creates a new point on the specified curve."] # [corresponds (EC_POINT_new)] pub fn new (group : & EcGroupRef) -> Result < EcPoint , ErrorStack > { unsafe { cvt_p (ffi :: EC_POINT_new (group . as_ptr ())) . map (EcPoint) } } # [doc = " Creates point from a binary representation"] # [corresponds (EC_POINT_oct2point)] pub fn from_bytes (group : & EcGroupRef , buf : & [u8] , ctx : & mut BigNumContextRef ,) -> Result < EcPoint , ErrorStack > { let point = EcPoint :: new (group) ? ; unsafe { cvt (ffi :: EC_POINT_oct2point (group . as_ptr () , point . as_ptr () , buf . as_ptr () , buf . len () , ctx . as_ptr () ,)) ? ; } Ok (point) } # [doc = " Creates point from a hexadecimal string representation"] # [corresponds (EC_POINT_hex2point)] # [cfg (not (any (boringssl , awslc)))] pub fn from_hex_str (group : & EcGroupRef , s : & str , ctx : & mut BigNumContextRef ,) -> Result < EcPoint , ErrorStack > { let point = EcPoint :: new (group) ? ; unsafe { let c_str = CString :: new (s . as_bytes ()) . unwrap () ; cvt_p (ffi :: EC_POINT_hex2point (group . as_ptr () , c_str . as_ptr () as * const _ , point . as_ptr () , ctx . as_ptr () ,)) ? ; } Ok (point) } }
};
}
