macro_rules! deps {
    () => {
        Regex!();
    };
}

macro_rules! impl_82 {
    () => {
        deps!();
        impl core :: fmt :: Debug for Regex { # [doc = " Shows the original regular expression."] fn fmt (& self , f : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { f . debug_tuple ("Regex") . field (& self . as_str ()) . finish () } }
    };
}

impl_82!()