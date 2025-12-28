macro_rules! deps {
    () => {
        RawIntoIter!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        # [cfg (not (feature = "nightly"))] impl < T , A : Allocator > Drop for RawIntoIter < T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { unsafe { self . iter . drop_elements () ; if let Some ((ptr , layout , ref alloc)) = self . allocation { alloc . deallocate (ptr , layout) ; } } } }
    };
}

impl_92!();