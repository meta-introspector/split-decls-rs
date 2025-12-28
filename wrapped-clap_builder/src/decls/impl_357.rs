macro_rules! deps {
    () => {
        Command!();
        Flag!();
        Arg!();
    };
}

macro_rules! impl_357 {
    () => {
        deps!();
        impl Ord for Flag < '_ > { fn cmp (& self , other : & Self) -> Ordering { match (self , other) { (Flag :: Command (s1 , _) , Flag :: Command (s2 , _)) | (Flag :: Arg (s1 , _) , Flag :: Arg (s2 , _)) | (Flag :: Command (s1 , _) , Flag :: Arg (s2 , _)) | (Flag :: Arg (s1 , _) , Flag :: Command (s2 , _)) => { if s1 == s2 { Ordering :: Equal } else { s1 . cmp (s2) } } } } }
    };
}

impl_357!()