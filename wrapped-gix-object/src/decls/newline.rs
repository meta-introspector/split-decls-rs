macro_rules! newline {
    () => {
        pub (crate) fn newline < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < & 'a [u8] , E > { alt ((b"\n" , b"\r\n")) . parse_next (i) }
    };
}

newline!();