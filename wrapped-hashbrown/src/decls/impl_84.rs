macro_rules! deps {
    () => {
        RawTable!();
        FullBucketsIndices!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Iterator for FullBucketsIndices { type Item = usize ; # [doc = " Advances the iterator and returns the next value. It is up to"] # [doc = " the caller to ensure that the `RawTable` outlives the `FullBucketsIndices`,"] # [doc = " because we cannot make the `next` method unsafe."] # [inline (always)] fn next (& mut self) -> Option < usize > { if self . items == 0 { return None ; } let nxt = unsafe { self . next_impl () } ; debug_assert ! (nxt . is_some ()) ; self . items -= 1 ; nxt } # [inline (always)] fn size_hint (& self) -> (usize , Option < usize >) { (self . items , Some (self . items)) } }
    };
}

impl_84!()