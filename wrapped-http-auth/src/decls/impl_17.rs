macro_rules! deps {
    () => {
        ChallengeParser!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl std :: iter :: FusedIterator for ChallengeParser < '_ > { }
    };
}

impl_17!()