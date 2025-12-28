macro_rules! take_newlines1 {
    () => {
        fn take_newlines1 < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i BStr , NomError < & 'i [u8] > > { repeat (1 .. 1024 , alt (("\r\n" , "\n"))) . map (| () | ()) . take () . map (bstr :: ByteSlice :: as_bstr) . parse_next (i) }
    };
}

take_newlines1!();