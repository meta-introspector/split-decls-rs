macro_rules! deps {
    () => {
        FluentValue!();
    };
}

macro_rules! impl_97 {
    () => {
        deps!();
        impl PartialEq for FluentValue < '_ > { fn eq (& self , other : & Self) -> bool { match (self , other) { (FluentValue :: String (s) , FluentValue :: String (s2)) => s == s2 , (FluentValue :: Number (s) , FluentValue :: Number (s2)) => s == s2 , (FluentValue :: Custom (s) , FluentValue :: Custom (s2)) => s == s2 , _ => false , } } }
    };
}

impl_97!();