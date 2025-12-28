macro_rules! deps {
    () => {
        Url!();
        InputScheme!();
        Error!();
    };
}

macro_rules! parse {
    () => {
        deps!();
        # [doc = " Parse the given `bytes` as a [git url](Url)."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " We cannot and should never have to deal with UTF-16 encoded windows strings, so bytes input is acceptable."] # [doc = " For file-paths, we don't expect UTF8 encoding either."] pub fn parse (input : & BStr) -> Result < Url , parse :: Error > { use parse :: InputScheme ; match parse :: find_scheme (input) { InputScheme :: Local => parse :: local (input) , InputScheme :: Url { protocol_end } if input [.. protocol_end] . eq_ignore_ascii_case (b"file") => { parse :: file_url (input , protocol_end) } InputScheme :: Url { protocol_end } => parse :: url (input , protocol_end) , InputScheme :: Scp { colon } => parse :: scp (input , colon) , } }
    };
}

parse!()