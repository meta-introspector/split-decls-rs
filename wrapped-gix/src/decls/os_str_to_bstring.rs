macro_rules! os_str_to_bstring {
    () => {
        # [doc = " Convert the given `input` into a `BString`, useful for usage in `clap`."] pub fn os_str_to_bstring (input : & OsStr) -> Option < BString > { Vec :: from_os_string (input . into ()) . map (Into :: into) . ok () }
    };
}

os_str_to_bstring!();