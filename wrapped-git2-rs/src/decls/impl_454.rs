macro_rules! deps {
    () => {
        MessageTrailersStrsIterator!();
    };
}

macro_rules! impl_454 {
    () => {
        deps!();
        impl FusedIterator for MessageTrailersStrsIterator < '_ > { }
    };
}

impl_454!();