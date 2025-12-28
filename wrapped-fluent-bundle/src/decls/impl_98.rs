macro_rules! deps {
    () => {
        FluentType!();
        FluentValue!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl Clone for FluentValue < '_ > { fn clone (& self) -> Self { match self { FluentValue :: String (s) => FluentValue :: String (s . clone ()) , FluentValue :: Number (s) => FluentValue :: Number (s . clone ()) , FluentValue :: Custom (s) => { let new_value : Box < dyn FluentType + Send > = s . duplicate () ; FluentValue :: Custom (new_value) } FluentValue :: Error => FluentValue :: Error , FluentValue :: None => FluentValue :: None , } } }
    };
}

impl_98!();