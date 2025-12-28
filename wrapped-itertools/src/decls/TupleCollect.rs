macro_rules! TupleCollect {
    () => {
        pub trait TupleCollect : Sized { type Item ; type Buffer : Default + AsRef < [Option < Self :: Item >] > + AsMut < [Option < Self :: Item >] > ; fn buffer_len (buf : & Self :: Buffer) -> usize { let s = buf . as_ref () ; s . iter () . position (Option :: is_none) . unwrap_or (s . len ()) } fn collect_from_iter < I > (iter : I , buf : & mut Self :: Buffer) -> Option < Self > where I : IntoIterator < Item = Self :: Item > ; fn collect_from_iter_no_buf < I > (iter : I) -> Option < Self > where I : IntoIterator < Item = Self :: Item > ; fn num_items () -> usize ; fn left_shift_push (& mut self , item : Self :: Item) ; }
    };
}

TupleCollect!();