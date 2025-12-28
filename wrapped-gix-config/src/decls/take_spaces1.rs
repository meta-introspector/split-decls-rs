macro_rules! take_spaces1 {
    () => {
        fn take_spaces1 < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i BStr , NomError < & 'i [u8] > > { take_while (1 .. , winnow :: stream :: AsChar :: is_space) . map (bstr :: ByteSlice :: as_bstr) . parse_next (i) }
    };
}

take_spaces1!()