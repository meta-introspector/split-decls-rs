macro_rules! deps {
    () => {
        EngineWrapper!();
        DecodeError!();
    };
}

macro_rules! decode_padding_starts_before_final_chunk_error_invalid_byte_at_first_pad {
    () => {
        deps!();
        # [doc = " Any amount of padding before final chunk that crosses over into final chunk with 1-4 bytes ="] # [doc = " invalid byte at first pad byte."] # [doc = " From this we know the padding must start in the final chunk."] # [apply (all_engines)] fn decode_padding_starts_before_final_chunk_error_invalid_byte_at_first_pad < E : EngineWrapper > (engine_wrapper : E ,) { let mut rng = seeded_rng () ; let prefix_quads_range = distributions :: Uniform :: from (1 .. 256) ; let suffix_pad_len_range = distributions :: Uniform :: from (1 ..= 4) ; for mode in pad_modes_allowing_padding () { let engine = E :: standard_with_pad_mode (true , mode) ; for _ in 0 .. 100_000 { let suffix_len = suffix_pad_len_range . sample (& mut rng) ; let mut encoded = "AAAA" . repeat (prefix_quads_range . sample (& mut rng)) . into_bytes () ; encoded . resize (encoded . len () + suffix_len , PAD_BYTE) ; let padding_len = rng . gen_range (suffix_len + 1 .. encoded . len ()) ; let padding_start = encoded . len () - padding_len ; encoded [padding_start ..] . fill (PAD_BYTE) ; assert_eq ! (Err (DecodeError :: InvalidByte (padding_start , PAD_BYTE)) , engine . decode (& encoded) , "suffix_len: {}, padding_len: {}, b64: {}" , suffix_len , padding_len , std :: str :: from_utf8 (& encoded) . unwrap ()) ; } } }
    };
}

decode_padding_starts_before_final_chunk_error_invalid_byte_at_first_pad!()