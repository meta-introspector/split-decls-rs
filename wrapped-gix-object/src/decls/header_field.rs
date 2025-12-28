macro_rules! header_field {
    () => {
        pub (crate) fn header_field < 'a , T , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8] , name : & 'static [u8] , parse_value : impl ModalParser < & 'a [u8] , T , E > ,) -> ModalResult < T , E > { terminated (preceded (terminated (name , SPACE) , parse_value) , NL) . parse_next (i) }
    };
}

header_field!()