macro_rules! escape_linker_output {
    () => {
        # [doc = " If the output of the msvc linker is not UTF-8 and the host is Windows,"] # [doc = " then try to convert the string from the OEM encoding."] # [cfg (windows)] fn escape_linker_output (s : & [u8] , flavour : LinkerFlavor) -> String { if flavour != LinkerFlavor :: Msvc (Lld :: No) { return escape_string (s) ; } match str :: from_utf8 (s) { Ok (s) => return s . to_owned () , Err (_) => match win :: locale_byte_str_to_string (s , win :: oem_code_page ()) { Some (s) => s , None => format ! ("Non-UTF-8 output: {}" , s . escape_ascii ()) , } , } }
    };
}

escape_linker_output!()