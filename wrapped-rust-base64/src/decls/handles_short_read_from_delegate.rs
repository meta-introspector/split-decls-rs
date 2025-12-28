macro_rules! deps {
    () => {
        RandomShortRead!();
        DecoderReader!();
    };
}

macro_rules! handles_short_read_from_delegate {
    () => {
        deps!();
        # [test] fn handles_short_read_from_delegate () { let mut rng = rand :: thread_rng () ; let mut bytes = Vec :: new () ; let mut b64 = String :: new () ; let mut decoded = Vec :: new () ; for _ in 0 .. 10_000 { bytes . clear () ; b64 . clear () ; decoded . clear () ; let size = rng . gen_range (0 .. (10 * BUF_SIZE)) ; bytes . extend (iter :: repeat (0) . take (size)) ; bytes . truncate (size) ; rng . fill_bytes (& mut bytes [.. size]) ; assert_eq ! (size , bytes . len ()) ; let engine = random_engine (& mut rng) ; engine . encode_string (& bytes [..] , & mut b64) ; let mut wrapped_reader = io :: Cursor :: new (b64 . as_bytes ()) ; let mut short_reader = RandomShortRead { delegate : & mut wrapped_reader , rng : & mut rng , } ; let mut decoder = DecoderReader :: new (& mut short_reader , & engine) ; let decoded_len = decoder . read_to_end (& mut decoded) . unwrap () ; assert_eq ! (size , decoded_len) ; assert_eq ! (& bytes [..] , & decoded [..]) ; } }
    };
}

handles_short_read_from_delegate!()