// Generated macro for impl_354 (impl)
macro_rules! Depcrate_ecimpl_354 {
() => {
// Module: crate::ec
// Provides: {"impl_354"}
// Dependencies: {}
impl EcKey < Params > { # [doc = " Constructs an `EcKey` corresponding to a known curve."] # [doc = ""] # [doc = " It will not have an associated public or private key. This kind of key is primarily useful"] # [doc = " to be provided to the `set_tmp_ecdh` methods on `Ssl` and `SslContextBuilder`."] # [corresponds (EC_KEY_new_by_curve_name)] pub fn from_curve_name (nid : Nid) -> Result < EcKey < Params > , ErrorStack > { unsafe { init () ; cvt_p (ffi :: EC_KEY_new_by_curve_name (nid . as_raw ())) . map (| p | EcKey :: from_ptr (p)) } } # [doc = " Constructs an `EcKey` corresponding to a curve."] # [corresponds (EC_KEY_set_group)] pub fn from_group (group : & EcGroupRef) -> Result < EcKey < Params > , ErrorStack > { unsafe { cvt_p (ffi :: EC_KEY_new ()) . map (| p | EcKey :: from_ptr (p)) . and_then (| key | { cvt (ffi :: EC_KEY_set_group (key . as_ptr () , group . as_ptr ())) . map (| _ | key) }) } } }
};
}
