macro_rules! deps {
    () => {
        DecodeError!();
        ShortRead!();
        DecoderReader!();
    };
}

macro_rules! internal_padding_error_with_short_read_concatenated_texts_invalid_byte_error {
    () => {
        deps!();
        # [test] fn internal_padding_error_with_short_read_concatenated_texts_invalid_byte_error () { let mut rng = rand :: thread_rng () ; let mut bytes = Vec :: new () ; let mut b64 = String :: new () ; let mut reader_decoded = Vec :: new () ; let mut bulk_decoded = Vec :: new () ; let engine = STANDARD ; for _ in 0 .. 10_000 { bytes . clear () ; b64 . clear () ; reader_decoded . clear () ; bulk_decoded . clear () ; let size = rng . gen_range (2 .. (10 * BUF_SIZE)) ; bytes . resize (size , 0) ; rng . fill_bytes (& mut bytes [.. size]) ; let split = loop { let s = rng . gen_range (1 .. size) ; if s % 3 != 0 { break s ; } ; } ; engine . encode_string (& bytes [.. split] , & mut b64) ; assert ! (b64 . contains ('=') , "split: {}, b64: {}" , split , b64) ; let bad_byte_pos = b64 . find ('=') . unwrap () ; engine . encode_string (& bytes [split ..] , & mut b64) ; let b64_bytes = b64 . as_bytes () ; let read_len = rng . gen_range (1 .. 10) ; let mut wrapped_reader = ShortRead { max_read_len : read_len , delegate : io :: Cursor :: new (& b64_bytes) , } ; let mut decoder = DecoderReader :: new (& mut wrapped_reader , & engine) ; let read_decode_err = decoder . read_to_end (& mut reader_decoded) . map_err (| e | { * e . into_inner () . and_then (| e | e . downcast :: < DecodeError > () . ok ()) . unwrap () }) . unwrap_err () ; let bulk_decode_err = engine . decode_vec (b64_bytes , & mut bulk_decoded) . unwrap_err () ; assert_eq ! (bulk_decode_err , read_decode_err , "read len: {}, bad byte pos: {}, b64: {}" , read_len , bad_byte_pos , std :: str :: from_utf8 (b64_bytes) . unwrap ()) ; assert_eq ! (DecodeError :: InvalidByte (split / 3 * 4 + match split % 3 { 1 => 2 , 2 => 3 , _ => unreachable ! () , } , PAD_BYTE) , read_decode_err) ; } }
    };
}

internal_padding_error_with_short_read_concatenated_texts_invalid_byte_error!();