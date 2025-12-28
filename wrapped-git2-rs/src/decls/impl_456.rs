macro_rules! deps {
    () => {
        MessageTrailersStrsIterator!();
    };
}

macro_rules! impl_456 {
    () => {
        deps!();
        impl DoubleEndedIterator for MessageTrailersStrsIterator < '_ > { fn next_back (& mut self) -> Option < Self :: Item > { self . 0 . range . next_back () . map (| index | to_str_tuple (& self . 0 . trailers , index)) } }
    };
}

impl_456!();