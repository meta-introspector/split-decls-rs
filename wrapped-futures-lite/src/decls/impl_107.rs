macro_rules! deps {
    () => {
        TryFoldFuture!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl < S , F , B > Unpin for TryFoldFuture < '_ , S , F , B > { }
    };
}

impl_107!()