macro_rules! deps {
    () => {
        DecoderReader!();
        RandomShortRead!();
    };
}

macro_rules! read_in_short_increments_with_short_delegate_reads {
    () => {
        deps!();
        # [test] fn read_in_short_increments_with_short_delegate_reads () { let mut rng = rand :: thread_rng () ; let mut bytes = Vec :: new () ; let mut b64 = String :: new () ; let mut decoded = Vec :: new () ; for _ in 0 .. 10_000 { bytes . clear () ; b64 . clear () ; decoded . clear () ; let size = rng . gen_range (0 .. (10 * BUF_SIZE)) ; bytes . extend (iter :: repeat (0) . take (size)) ; decoded . extend (iter :: repeat (0) . take (size * 3)) ; rng . fill_bytes (& mut bytes [..]) ; assert_eq ! (size , bytes . len ()) ; let engine = random_engine (& mut rng) ; engine . encode_string (& bytes [..] , & mut b64) ; let mut base_reader = io :: Cursor :: new (& b64 [..]) ; let mut decoder = DecoderReader :: new (& mut base_reader , & engine) ; let mut short_reader = RandomShortRead { delegate : & mut decoder , rng : & mut rand :: thread_rng () , } ; consume_with_short_reads_and_validate (& mut rng , & bytes [..] , & mut decoded , & mut short_reader ,) ; } }
    };
}

read_in_short_increments_with_short_delegate_reads!()