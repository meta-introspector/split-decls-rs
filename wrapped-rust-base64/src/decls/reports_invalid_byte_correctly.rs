macro_rules! deps {
    () => {
        GeneralPurpose!();
        DecodeError!();
        DecoderReader!();
    };
}

macro_rules! reports_invalid_byte_correctly {
    () => {
        deps!();
        # [test] fn reports_invalid_byte_correctly () { let mut rng = rand :: thread_rng () ; let mut bytes = Vec :: new () ; let mut b64 = String :: new () ; let mut stream_decoded = Vec :: new () ; let mut bulk_decoded = Vec :: new () ; for _ in 0 .. 10_000 { bytes . clear () ; b64 . clear () ; stream_decoded . clear () ; bulk_decoded . clear () ; let size = rng . gen_range (1 .. (10 * BUF_SIZE)) ; bytes . extend (iter :: repeat (0) . take (size)) ; rng . fill_bytes (& mut bytes [.. size]) ; assert_eq ! (size , bytes . len ()) ; let engine = GeneralPurpose :: new (& alphabet :: STANDARD , random_config (& mut rng)) ; engine . encode_string (& bytes [..] , & mut b64) ; let bad_byte_pos = rng . gen_range (0 .. b64 . len ()) ; let mut b64_bytes = b64 . bytes () . collect :: < Vec < u8 > > () ; b64_bytes [bad_byte_pos] = b'*' ; let mut wrapped_reader = io :: Cursor :: new (b64_bytes . clone ()) ; let mut decoder = DecoderReader :: new (& mut wrapped_reader , & engine) ; let read_decode_err = decoder . read_to_end (& mut stream_decoded) . map_err (| e | { let kind = e . kind () ; let inner = e . into_inner () . and_then (| e | e . downcast :: < DecodeError > () . ok ()) ; inner . map (| i | (* i , kind)) }) . err () . and_then (| o | o) ; let bulk_decode_err = engine . decode_vec (& b64_bytes [..] , & mut bulk_decoded) . err () ; assert_eq ! (bulk_decode_err . map (| e | (e , io :: ErrorKind :: InvalidData)) , read_decode_err) ; } }
    };
}

reports_invalid_byte_correctly!();