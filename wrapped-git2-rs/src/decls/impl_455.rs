macro_rules! deps {
    () => {
        MessageTrailersStrsIterator!();
    };
}

macro_rules! impl_455 {
    () => {
        deps!();
        impl ExactSizeIterator for MessageTrailersStrsIterator < '_ > { fn len (& self) -> usize { self . 0 . range . len () } }
    };
}

impl_455!()