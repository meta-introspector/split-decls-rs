macro_rules! deps {
    () => {
        MessageTrailersBytesIterator!();
    };
}

macro_rules! impl_460 {
    () => {
        deps!();
        impl FusedIterator for MessageTrailersBytesIterator < '_ > { }
    };
}

impl_460!();