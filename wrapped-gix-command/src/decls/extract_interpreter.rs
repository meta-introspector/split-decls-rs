macro_rules! extract_interpreter {
    () => {
        # [doc = " Parse the shebang (`#!<path>`) from the first line of `executable`, and return the shebang"] # [doc = " data when available."] pub fn extract_interpreter (executable : & Path) -> Option < shebang :: Data > { # [cfg (windows)] if is_exe (executable) { return None ; } let mut buf = [0 ; 100] ; let mut file = std :: fs :: File :: open (executable) . ok () ? ; let n = file . read (& mut buf) . ok () ? ; shebang :: parse (buf [.. n] . as_bstr ()) }
    };
}

extract_interpreter!()