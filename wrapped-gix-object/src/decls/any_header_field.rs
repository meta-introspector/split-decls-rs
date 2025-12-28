macro_rules! any_header_field {
    () => {
        pub (crate) fn any_header_field < 'a , T , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8] , parse_value : impl ModalParser < & 'a [u8] , T , E > ,) -> ModalResult < (& 'a [u8] , T) , E > { terminated ((terminated (take_till (1 .. , SPACE_OR_NL) , SPACE) , parse_value) , NL) . parse_next (i) }
    };
}

any_header_field!()