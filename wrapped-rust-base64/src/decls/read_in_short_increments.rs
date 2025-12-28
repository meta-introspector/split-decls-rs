macro_rules! deps {
    () => {
        DecoderReader!();
    };
}

macro_rules! read_in_short_increments {
    () => {
        deps!();
        # [test] fn read_in_short_increments () { let mut rng = rand :: thread_rng () ; let mut bytes = Vec :: new () ; let mut b64 = String :: new () ; let mut decoded = Vec :: new () ; for _ in 0 .. 10_000 { bytes . clear () ; b64 . clear () ; decoded . clear () ; let size = rng . gen_range (0 .. (10 * BUF_SIZE)) ; bytes . extend (iter :: repeat (0) . take (size)) ; decoded . extend (iter :: repeat (0) . take (size * 3)) ; rng . fill_bytes (& mut bytes [..]) ; assert_eq ! (size , bytes . len ()) ; let engine = random_engine (& mut rng) ; engine . encode_string (& bytes [..] , & mut b64) ; let mut wrapped_reader = io :: Cursor :: new (& b64 [..]) ; let mut decoder = DecoderReader :: new (& mut wrapped_reader , & engine) ; consume_with_short_reads_and_validate (& mut rng , & bytes [..] , & mut decoded , & mut decoder) ; } }
    };
}

read_in_short_increments!()