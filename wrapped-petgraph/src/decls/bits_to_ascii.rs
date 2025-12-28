macro_rules! bits_to_ascii {
    () => {
        fn bits_to_ascii (mut bits : Vec < usize >) -> String { while bits . len () % 6 != 0 { bits . push (0) ; } let bits_strs = bits . iter () . map (| bit | bit . to_string ()) . collect :: < Vec < _ > > () ; let bytes = bits_strs . chunks (6) . map (| bits_chunk | bits_chunk . join ("")) . map (| bits_str | usize :: from_str_radix (& bits_str , 2)) ; bytes . map (| byte | char :: from ((N + byte . unwrap ()) as u8)) . collect () }
    };
}

bits_to_ascii!()