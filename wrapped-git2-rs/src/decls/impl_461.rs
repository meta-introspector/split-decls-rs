macro_rules! deps {
    () => {
        MessageTrailersBytesIterator!();
    };
}

macro_rules! impl_461 {
    () => {
        deps!();
        impl ExactSizeIterator for MessageTrailersBytesIterator < '_ > { fn len (& self) -> usize { self . 0 . range . len () } }
    };
}

impl_461!();