macro_rules! CargoConfigFileReader {
    () => {
        pub (crate) struct CargoConfigFileReader < 'a > { toml_str : & 'a str , line_ends : Vec < usize > , table : Spanned < DeTable < 'a > > , }
    };
}

CargoConfigFileReader!();