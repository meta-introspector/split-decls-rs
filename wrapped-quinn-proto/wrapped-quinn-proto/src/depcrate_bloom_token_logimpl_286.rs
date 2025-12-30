// Generated macro for impl_286 (impl)
macro_rules! Depcrate_bloom_token_logimpl_286 {
() => {
// Module: crate::bloom_token_log
// Provides: {"impl_286"}
// Dependencies: {}
impl TokenLog for BloomTokenLog { fn check_and_insert (& self , nonce : u128 , issued : SystemTime , lifetime : Duration ,) -> Result < () , TokenReuseError > { trace ! (% nonce , "check_and_insert") ; if lifetime . is_zero () { return Err (TokenReuseError) ; } let mut guard = self . 0 . lock () . unwrap () ; let state = & mut * guard ; let expires_at = issued + lifetime ; let Ok (periods_forward) = expires_at . duration_since (state . period_1_start) . map (| duration | duration . as_nanos () / lifetime . as_nanos ()) else { warn ! ("BloomTokenLog presented with token too far in past") ; return Err (TokenReuseError) ; } ; let filter = match periods_forward { 0 => & mut state . filter_1 , 1 => & mut state . filter_2 , 2 => { state . filter_1 = take (& mut state . filter_2) ; state . period_1_start += lifetime ; & mut state . filter_2 } _ => { state . filter_1 = Filter :: default () ; state . filter_2 = Filter :: default () ; state . period_1_start = expires_at ; & mut state . filter_1 } } ; filter . check_and_insert (nonce as u64 , & state . config) } }
};
}
