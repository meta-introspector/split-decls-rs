macro_rules! deps {
    () => {
        MessageTrailersStrsIterator!();
    };
}

macro_rules! impl_453 {
    () => {
        deps!();
        impl < 'pair > Iterator for MessageTrailersStrsIterator < 'pair > { type Item = (& 'pair str , & 'pair str) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . range . next () . map (| index | to_str_tuple (& self . 0 . trailers , index)) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . range . size_hint () } }
    };
}

impl_453!()