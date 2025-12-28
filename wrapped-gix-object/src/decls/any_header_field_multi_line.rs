macro_rules! any_header_field_multi_line {
    () => {
        pub (crate) fn any_header_field_multi_line < 'a , E : ParserError < & 'a [u8] > + AddContext < & 'a [u8] , StrContext > > (i : & mut & 'a [u8] ,) -> ModalResult < (& 'a [u8] , BString) , E > { (terminated (take_till (1 .. , SPACE_OR_NL) , SPACE) , (take_till (0 .. , NL) , NL , repeat (1 .. , terminated ((SPACE , take_until (0 .. , NL)) , NL)) . map (| () | ()) ,) . take () . map (| o : & [u8] | { let bytes = o . as_bstr () ; let mut out = BString :: from (Vec :: with_capacity (bytes . len ())) ; let mut lines = bytes . lines_with_terminator () ; out . push_str (lines . next () . expect ("first line")) ; for line in lines { out . push_str (& line [1 ..]) ; } out }) ,) . context (StrContext :: Expected ("name <multi-line-value>" . into ())) . parse_next (i) }
    };
}

any_header_field_multi_line!()