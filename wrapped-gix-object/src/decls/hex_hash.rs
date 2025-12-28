macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! hex_hash {
    () => {
        deps!();
        pub fn hex_hash < 'a , E : ParserError < & 'a [u8] > > (i : & mut & 'a [u8]) -> ModalResult < & 'a BStr , E > { take_while (gix_hash :: Kind :: shortest () . len_in_hex () ..= gix_hash :: Kind :: longest () . len_in_hex () , is_hex_digit_lc ,) . map (ByteSlice :: as_bstr) . parse_next (i) }
    };
}

hex_hash!();