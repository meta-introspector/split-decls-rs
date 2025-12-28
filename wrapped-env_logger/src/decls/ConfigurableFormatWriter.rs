macro_rules! deps {
    () => {
        ConfigurableFormat!();
        Formatter!();
    };
}

macro_rules! ConfigurableFormatWriter {
    () => {
        deps!();
        # [doc = " The default format."] # [doc = ""] # [doc = " This format needs to work with any combination of crate features."] struct ConfigurableFormatWriter < 'a > { format : & 'a ConfigurableFormat , buf : & 'a mut Formatter , written_header_value : bool , }
    };
}

ConfigurableFormatWriter!();