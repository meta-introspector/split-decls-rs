// Generated macro for store_param (function)
macro_rules! Depcrate_digeststore_param {
() => {
// Module: crate::digest
// Provides: {"store_param"}
// Dependencies: {}
# [doc = " Helper for `DigestClient::try_from` which stashes away a `&ParamValue`."] # [inline (never)] fn store_param < 'v , 'tmp > (k : & 'tmp str , v : & 'v ParamValue < 'v > , expected_k : & 'tmp str , set_v : & 'tmp mut Option < & 'v ParamValue < 'v > > , add_len : & 'tmp mut usize ,) -> Result < bool , String > { if ! k . eq_ignore_ascii_case (expected_k) { return Ok (false) ; } if set_v . is_some () { return Err (format ! ("duplicate parameter {:?}" , k)) ; } * add_len += v . unescaped_len () ; * set_v = Some (v) ; Ok (true) }
};
}
