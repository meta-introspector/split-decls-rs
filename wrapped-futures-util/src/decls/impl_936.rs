macro_rules! deps {
    () => {
        Flush!();
    };
}

macro_rules! impl_936 {
    () => {
        deps!();
        impl < Si : Unpin + ? Sized , Item > Unpin for Flush < '_ , Si , Item > { }
    };
}

impl_936!();