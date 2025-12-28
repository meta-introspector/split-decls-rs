macro_rules! get_bits_as_decimal {
    () => {
        fn get_bits_as_decimal (bits : Vec < u8 >) -> usize { let bits_str = bits . iter () . map (| bit | bit . to_string ()) . collect :: < Vec < String > > () . join ("") ; usize :: from_str_radix (& bits_str , 2) . unwrap () }
    };
}

get_bits_as_decimal!();