macro_rules! deps {
    () => {
        PartialInterruptingWriter!();
        EncoderWriter!();
    };
}

macro_rules! writes_that_only_write_part_of_input_and_sometimes_interrupt_produce_correct_encoded_data {
    () => {
        deps!();
        # [test] fn writes_that_only_write_part_of_input_and_sometimes_interrupt_produce_correct_encoded_data () { let mut rng = rand :: thread_rng () ; let mut orig_data = Vec :: < u8 > :: new () ; let mut stream_encoded = Vec :: < u8 > :: new () ; let mut normal_encoded = String :: new () ; for _ in 0 .. 1_000 { orig_data . clear () ; stream_encoded . clear () ; normal_encoded . clear () ; let orig_len : usize = rng . gen_range (100 .. 20_000) ; for _ in 0 .. orig_len { orig_data . push (rng . gen ()) ; } let engine = random_engine (& mut rng) ; engine . encode_string (& orig_data , & mut normal_encoded) ; { let mut partial_rng = rand :: thread_rng () ; let mut partial_writer = PartialInterruptingWriter { w : & mut stream_encoded , rng : & mut partial_rng , full_input_fraction : 0.1 , no_interrupt_fraction : 0.1 , } ; let mut stream_encoder = EncoderWriter :: new (& mut partial_writer , & engine) ; let mut bytes_consumed = 0 ; while bytes_consumed < orig_len { let input_len : usize = cmp :: min (rng . gen_range (0 .. 100) , orig_len - bytes_consumed) ; let res = stream_encoder . write (& orig_data [bytes_consumed .. bytes_consumed + input_len]) ; match res { Ok (len) => bytes_consumed += len , Err (e) => match e . kind () { io :: ErrorKind :: Interrupted => continue , _ => { panic ! ("should not see other errors") ; } } , } } let _ = stream_encoder . finish () . unwrap () ; assert_eq ! (orig_len , bytes_consumed) ; } assert_eq ! (normal_encoded , str :: from_utf8 (& stream_encoded) . unwrap ()) ; } }
    };
}

writes_that_only_write_part_of_input_and_sometimes_interrupt_produce_correct_encoded_data!();