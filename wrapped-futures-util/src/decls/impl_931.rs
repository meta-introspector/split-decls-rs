macro_rules! deps {
    () => {
        Feed!();
    };
}

macro_rules! impl_931 {
    () => {
        deps!();
        impl < Si : Unpin + ? Sized , Item > Unpin for Feed < '_ , Si , Item > { }
    };
}

impl_931!();