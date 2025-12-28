macro_rules! deps {
    () => {
        KnownArgumentNames!();
    };
}

macro_rules! impl_198 {
    () => {
        deps!();
        impl KnownArgumentNames < '_ > { fn get_suggestion (& self , name : & str) -> String { make_suggestion (" Did you mean" , self . current_args . iter () . map (| (args , _) | args . iter () . map (| arg | arg . 0 . as_str ())) . flatten () , name ,) . unwrap_or_default () } }
    };
}

impl_198!();