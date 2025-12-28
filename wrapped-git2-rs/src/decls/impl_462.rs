macro_rules! deps {
    () => {
        MessageTrailersBytesIterator!();
    };
}

macro_rules! impl_462 {
    () => {
        deps!();
        impl DoubleEndedIterator for MessageTrailersBytesIterator < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . range . next_back () . map (| index | to_bytes_tuple (& self . 0 . trailers , index)) } }
    };
}

impl_462!();