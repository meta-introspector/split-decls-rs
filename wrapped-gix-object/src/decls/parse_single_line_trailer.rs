macro_rules! parse_single_line_trailer {
    () => {
        fn parse_single_line_trailer < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < (& 'a BStr , & 'a BStr) , E > { * i = i . trim_end () ; let (token , value) = separated_pair (take_until (1 .. , b":" . as_ref ()) , b": " , rest) . parse_next (i) ? ; if token . trim_end () . len () != token . len () || value . trim_start () . len () != value . len () { Err (winnow :: error :: ErrMode :: from_input (i) . cut ()) } else { Ok ((token . as_bstr () , value . as_bstr ())) } }
    };
}

parse_single_line_trailer!();