macro_rules! deps {
    () => {
        ValueHint!();
        Result!();
    };
}

macro_rules! impl_247 {
    () => {
        deps!();
        impl FromStr for ValueHint { type Err = String ; fn from_str (s : & str) -> Result < Self , < Self as FromStr > :: Err > { Ok (match & * s . to_ascii_lowercase () { "unknown" => ValueHint :: Unknown , "other" => ValueHint :: Other , "anypath" => ValueHint :: AnyPath , "filepath" => ValueHint :: FilePath , "dirpath" => ValueHint :: DirPath , "executablepath" => ValueHint :: ExecutablePath , "commandname" => ValueHint :: CommandName , "commandstring" => ValueHint :: CommandString , "commandwitharguments" => ValueHint :: CommandWithArguments , "username" => ValueHint :: Username , "hostname" => ValueHint :: Hostname , "url" => ValueHint :: Url , "emailaddress" => ValueHint :: EmailAddress , _ => return Err (format ! ("unknown ValueHint: `{s}`")) , }) } }
    };
}

impl_247!();