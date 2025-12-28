macro_rules! deps {
    () => {
        Convert!();
        MessageTrailersStrs!();
        MessageTrailersBytes!();
        MessageTrailersStrsIterator!();
    };
}

macro_rules! impl_443 {
    () => {
        deps!();
        impl MessageTrailersStrs { # [doc = " Create a borrowed iterator."] pub fn iter (& self) -> MessageTrailersStrsIterator < '_ > { MessageTrailersStrsIterator (self . 0 . iter ()) } # [doc = " The number of trailer key–value pairs."] pub fn len (& self) -> usize { self . 0 . len () } # [doc = " Convert to the “bytes” variant."] pub fn to_bytes (self) -> MessageTrailersBytes { MessageTrailersBytes (self . 0) } }
    };
}

impl_443!();