macro_rules! literal_to_string {
    () => {
        fn literal_to_string (lit : token :: Lit) -> String { let token :: Lit { kind , symbol , suffix } = lit ; let mut out = match kind { token :: Byte => format ! ("b'{symbol}'") , token :: Char => format ! ("'{symbol}'") , token :: Str => format ! ("\"{symbol}\"") , token :: StrRaw (n) => { format ! ("r{delim}\"{string}\"{delim}" , delim = "#" . repeat (n as usize) , string = symbol) } token :: ByteStr => format ! ("b\"{symbol}\"") , token :: ByteStrRaw (n) => { format ! ("br{delim}\"{string}\"{delim}" , delim = "#" . repeat (n as usize) , string = symbol) } token :: CStr => format ! ("c\"{symbol}\"") , token :: CStrRaw (n) => { format ! ("cr{delim}\"{symbol}\"{delim}" , delim = "#" . repeat (n as usize)) } token :: Integer | token :: Float | token :: Bool | token :: Err (_) => symbol . to_string () , } ; if let Some (suffix) = suffix { out . push_str (suffix . as_str ()) } out }
    };
}

literal_to_string!();