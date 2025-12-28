macro_rules! deps {
    () => {
        DecoderReader!();
        ShortRead!();
    };
}

macro_rules! internal_padding_anywhere_error {
    () => {
        deps!();
        # [test] fn internal_padding_anywhere_error () { let mut rng = rand :: thread_rng () ; let mut bytes = Vec :: new () ; let mut b64 = String :: new () ; let mut reader_decoded = Vec :: new () ; let engine = STANDARD ; for _ in 0 .. 10_000 { bytes . clear () ; b64 . clear () ; reader_decoded . clear () ; bytes . resize (10 * BUF_SIZE , 0) ; rng . fill_bytes (& mut bytes [..]) ; engine . encode_string (& bytes [..] , & mut b64) ; let mut b64_bytes = b64 . as_bytes () . to_vec () ; b64_bytes [rng . gen_range (0 .. bytes . len () - 4)] = PAD_BYTE ; let read_len = rng . gen_range (1 .. 10) ; let mut wrapped_reader = ShortRead { max_read_len : read_len , delegate : io :: Cursor :: new (& b64_bytes) , } ; let mut decoder = DecoderReader :: new (& mut wrapped_reader , & engine) ; let result = decoder . read_to_end (& mut reader_decoded) ; assert ! (result . is_err ()) ; } }
    };
}

internal_padding_anywhere_error!();