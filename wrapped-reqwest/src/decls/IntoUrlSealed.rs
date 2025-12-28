macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! IntoUrlSealed {
    () => {
        deps!();
        pub trait IntoUrlSealed { fn into_url (self) -> crate :: Result < Url > ; fn as_str (& self) -> & str ; }
    };
}

IntoUrlSealed!();