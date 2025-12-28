macro_rules! deps {
    () => {
        MessageTrailersBytesIterator!();
    };
}

macro_rules! impl_459 {
    () => {
        deps!();
        impl < 'pair > Iterator for MessageTrailersBytesIterator < 'pair > { type Item = (& 'pair [u8] , & 'pair [u8]) ; fn next (& mut self) -> Option < Self :: Item > { self . 0 . range . next () . map (| index | to_bytes_tuple (& self . 0 . trailers , index)) } fn size_hint (& self) -> (usize , Option < usize >) { self . 0 . range . size_hint () } }
    };
}

impl_459!();