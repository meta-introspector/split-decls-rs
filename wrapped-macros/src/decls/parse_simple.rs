macro_rules! parse_simple {
    () => {
        # [inline] const fn parse_simple (s : & [u8]) -> Result < [u8 ; 16] , () > { if s . len () != 32 { return Err (()) ; } let mut buf : [u8 ; 16] = [0 ; 16] ; let mut i = 0 ; while i < 16 { let h1 = HEX_TABLE [s [i * 2] as usize] ; let h2 = HEX_TABLE [s [i * 2 + 1] as usize] ; if h1 | h2 == 0xff { return Err (()) ; } buf [i] = SHL4_TABLE [h1 as usize] | h2 ; i += 1 ; } Ok (buf) }
    };
}

parse_simple!();