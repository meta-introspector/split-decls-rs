macro_rules! deps {
    () => {
        Command!();
    };
}

macro_rules! impl_3 {
    () => {
        deps!();
        impl Command { # [doc = " Produce the name of the command as known by the server side."] pub fn as_str (& self) -> & 'static str { match self { Command :: LsRefs => "ls-refs" , Command :: Fetch => "fetch" , } } }
    };
}

impl_3!()