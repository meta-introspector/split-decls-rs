macro_rules! deps {
    () => {
        InfoType!();
    };
}

macro_rules! debug {
    () => {
        deps!();
        pub fn debug (kind : InfoType , data : & [u8]) { let out = io :: stderr () ; let prefix = match kind { InfoType :: Text => "*" , InfoType :: HeaderIn => "<" , InfoType :: HeaderOut => ">" , InfoType :: DataIn | InfoType :: SslDataIn => "{" , InfoType :: DataOut | InfoType :: SslDataOut => "}" , } ; let mut out = out . lock () ; drop (write ! (out , "{} " , prefix)) ; match str :: from_utf8 (data) { Ok (s) => drop (out . write_all (s . as_bytes ())) , Err (_) => drop (writeln ! (out , "({} bytes of data)" , data . len ())) , } }
    };
}

debug!();