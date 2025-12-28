macro_rules! config_name {
    () => {
        # [doc = " Parses the config name of a config pair. Assumes the input has already been"] # [doc = " trimmed of any leading whitespace."] fn config_name < 'i > (i : & mut & 'i [u8]) -> ModalResult < & 'i BStr , NomError < & 'i [u8] > > { (one_of (| c : u8 | c . is_ascii_alphabetic ()) , take_while (0 .. , | c : u8 | c . is_ascii_alphanumeric () || c == b'-') ,) . take () . map (bstr :: ByteSlice :: as_bstr) . parse_next (i) }
    };
}

config_name!();