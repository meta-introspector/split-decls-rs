macro_rules! deps {
    () => {
        Encoding!();
        CoderResult!();
    };
}

macro_rules! decode_to_utf16_with_boundary {
    () => {
        deps!();
        pub fn decode_to_utf16_with_boundary (encoding : & 'static Encoding , head : & [u8] , tail : & [u8] , expect : & [u16] ,) { let mut decoder = encoding . new_decoder () ; let mut dest : Vec < u16 > = Vec :: with_capacity (decoder . max_utf16_buffer_length (head . len () + tail . len ()) . unwrap () ,) ; let capacity = dest . capacity () ; dest . resize (capacity , 0u16) ; let mut total_read = 0 ; let mut total_written = 0 ; { let (complete , read , written , _) = decoder . decode_to_utf16 (head , & mut dest , false) ; match complete { CoderResult :: InputEmpty => { } CoderResult :: OutputFull => { unreachable ! () ; } } total_read += read ; total_written += written ; } { let (complete , read , written , _) = decoder . decode_to_utf16 (tail , & mut dest [total_written ..] , true) ; match complete { CoderResult :: InputEmpty => { } CoderResult :: OutputFull => { unreachable ! () ; } } total_read += read ; total_written += written ; } assert_eq ! (total_read , head . len () + tail . len ()) ; assert_eq ! (total_written , expect . len ()) ; dest . truncate (total_written) ; assert_eq ! (& dest [..] , expect) ; }
    };
}

decode_to_utf16_with_boundary!();