macro_rules! deps {
    () => {
        Qop!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl Qop { # [doc = " Returns a string form as expected over the wire."] fn as_str (self) -> & 'static str { match self { Qop :: Auth => "auth" , Qop :: AuthInt => "auth-int" , } } }
    };
}

impl_28!();