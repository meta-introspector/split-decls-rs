macro_rules! deps {
    () => {
        Encoding!();
        CoderResult!();
    };
}

macro_rules! decode_to_utf8_with_boundary {
    () => {
        deps!();
        pub fn decode_to_utf8_with_boundary (encoding : & 'static Encoding , head : & [u8] , tail : & [u8] , expect : & str ,) { let mut decoder = encoding . new_decoder () ; let mut dest : Vec < u8 > = Vec :: with_capacity (decoder . max_utf8_buffer_length (head . len () + tail . len ()) . unwrap () ,) ; let capacity = dest . capacity () ; dest . resize (capacity , 0u8) ; let mut total_read = 0 ; let mut total_written = 0 ; { let (complete , read , written , _) = decoder . decode_to_utf8 (head , & mut dest , false) ; match complete { CoderResult :: InputEmpty => { } CoderResult :: OutputFull => { unreachable ! () ; } } total_read += read ; total_written += written ; } { let (complete , read , written , _) = decoder . decode_to_utf8 (tail , & mut dest [total_written ..] , true) ; match complete { CoderResult :: InputEmpty => { } CoderResult :: OutputFull => { unreachable ! () ; } } total_read += read ; total_written += written ; } assert_eq ! (total_read , head . len () + tail . len ()) ; assert_eq ! (total_written , expect . len ()) ; dest . truncate (total_written) ; assert_eq ! (& dest [..] , expect . as_bytes ()) ; }
    };
}

decode_to_utf8_with_boundary!()