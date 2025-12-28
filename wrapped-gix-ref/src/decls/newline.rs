macro_rules! newline {
    () => {
        pub fn newline < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < & 'a [u8] , E > { alt ((b"\r\n" , b"\n")) . parse_next (i) }
    };
}

newline!()