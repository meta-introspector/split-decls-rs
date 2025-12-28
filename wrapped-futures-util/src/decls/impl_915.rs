macro_rules! deps {
    () => {
        Close!();
    };
}

macro_rules! impl_915 {
    () => {
        deps!();
        impl < Si : Unpin + ? Sized , Item > Unpin for Close < '_ , Si , Item > { }
    };
}

impl_915!();