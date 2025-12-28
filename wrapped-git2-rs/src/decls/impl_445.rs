macro_rules! deps {
    () => {
        MessageTrailersBytesIterator!();
        MessageTrailersBytes!();
    };
}

macro_rules! impl_445 {
    () => {
        deps!();
        impl MessageTrailersBytes { # [doc = " Create a borrowed iterator."] pub fn iter (& self) -> MessageTrailersBytesIterator < '_ > { MessageTrailersBytesIterator (self . 0 . iter ()) } # [doc = " The number of trailer key–value pairs."] pub fn len (& self) -> usize { self . 0 . len () } }
    };
}

impl_445!()