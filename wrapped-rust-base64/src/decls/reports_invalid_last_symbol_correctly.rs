macro_rules! deps {
    () => {
        DecoderReader!();
        GeneralPurpose!();
        DecodeError!();
    };
}

macro_rules! reports_invalid_last_symbol_correctly {
    () => {
        deps!();
        # [test] fn reports_invalid_last_symbol_correctly () { let mut rng = rand :: thread_rng () ; let mut bytes = Vec :: new () ; let mut b64 = String :: new () ; let mut b64_bytes = Vec :: new () ; let mut decoded = Vec :: new () ; let mut bulk_decoded = Vec :: new () ; for _ in 0 .. 1_000 { bytes . clear () ; b64 . clear () ; b64_bytes . clear () ; let size = rng . gen_range (1 .. (10 * BUF_SIZE)) ; bytes . extend (iter :: repeat (0) . take (size)) ; decoded . extend (iter :: repeat (0) . take (size)) ; rng . fill_bytes (& mut bytes [..]) ; assert_eq ! (size , bytes . len ()) ; let config = random_config (& mut rng) ; let alphabet = random_alphabet (& mut rng) ; let engine = GeneralPurpose :: new (alphabet , config . with_encode_padding (false)) ; engine . encode_string (& bytes [..] , & mut b64) ; b64_bytes . extend (b64 . bytes ()) ; assert_eq ! (b64_bytes . len () , b64 . len ()) ; for & s1 in alphabet . symbols . iter () { decoded . clear () ; bulk_decoded . clear () ; * b64_bytes . last_mut () . unwrap () = s1 ; let bulk_res = engine . decode_vec (& b64_bytes [..] , & mut bulk_decoded) ; let mut wrapped_reader = io :: Cursor :: new (& b64_bytes [..]) ; let mut decoder = DecoderReader :: new (& mut wrapped_reader , & engine) ; let stream_res = decoder . read_to_end (& mut decoded) . map (| _ | ()) . map_err (| e | { e . into_inner () . and_then (| e | e . downcast :: < DecodeError > () . ok ()) }) ; assert_eq ! (bulk_res . map_err (| e | Some (Box :: new (e))) , stream_res) ; } } }
    };
}

reports_invalid_last_symbol_correctly!()