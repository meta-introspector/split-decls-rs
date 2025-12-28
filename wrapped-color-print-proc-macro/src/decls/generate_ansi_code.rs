macro_rules! generate_ansi_code {
    () => {
        # [doc = " Generate an SGR ANSI sequence."] pub fn generate_ansi_code (params : & [u8]) -> String { let mut ansi_code = String :: from ("\u{1b}[") ; let mut first = true ; for param in params { if first { first = false ; } else { ansi_code . push (';') ; } ansi_code . push_str (& format ! ("{}" , param)) ; } ansi_code . push ('m') ; ansi_code }
    };
}

generate_ansi_code!()