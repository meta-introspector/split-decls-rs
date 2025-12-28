macro_rules! deps {
    () => {
        Send!();
    };
}

macro_rules! impl_953 {
    () => {
        deps!();
        impl < Si : Unpin + ? Sized , Item > Unpin for Send < '_ , Si , Item > { }
    };
}

impl_953!();