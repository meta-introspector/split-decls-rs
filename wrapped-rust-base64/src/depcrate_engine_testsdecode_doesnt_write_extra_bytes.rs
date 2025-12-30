// Generated macro for decode_doesnt_write_extra_bytes (function)
macro_rules! Depcrate_engine_testsdecode_doesnt_write_extra_bytes {
() => {
// Module: crate::engine::tests
// Provides: {"decode_doesnt_write_extra_bytes"}
// Dependencies: {}
# [apply (all_engines)] fn decode_doesnt_write_extra_bytes < E > (engine_wrapper : E) where E : EngineWrapper , < < E as EngineWrapper > :: Engine as Engine > :: Config : fmt :: Debug , { let mut rng = seeded_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut encode_buf = Vec :: < u8 > :: new () ; let mut decode_buf = Vec :: < u8 > :: new () ; let mut decode_buf_backup = Vec :: < u8 > :: new () ; let len_range = distributions :: Uniform :: new (1 , 1_000) ; for _ in 0 .. 10_000 { let engine = E :: random (& mut rng) ; orig_data . clear () ; encode_buf . clear () ; decode_buf . clear () ; decode_buf_backup . clear () ; let orig_len = fill_rand (& mut orig_data , & mut rng , & len_range) ; encode_buf . resize (orig_len * 2 + 100 , 0) ; let encoded_len = engine . encode_slice (& orig_data [..] , & mut encode_buf [..]) . unwrap () ; encode_buf . truncate (encoded_len) ; let prefix_len = 1024 ; fill_rand_len (& mut decode_buf , & mut rng , prefix_len * 2 + orig_len * 2) ; decode_buf_backup . extend_from_slice (& decode_buf [..]) ; let dec_len = engine . decode_slice_unchecked (& encode_buf , & mut decode_buf [prefix_len ..]) . unwrap () ; assert_eq ! (orig_len , dec_len) ; assert_eq ! (& orig_data [..] , & decode_buf [prefix_len .. prefix_len + dec_len]) ; assert_eq ! (& decode_buf_backup [.. prefix_len] , & decode_buf [.. prefix_len]) ; assert_eq ! (& decode_buf_backup [prefix_len + dec_len ..] , & decode_buf [prefix_len + dec_len ..]) ; } }
};
}
