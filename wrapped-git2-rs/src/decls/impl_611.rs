macro_rules! deps {
    () => {
        References!();
        Reference!();
        ReferenceNames!();
    };
}

macro_rules! impl_611 {
    () => {
        deps!();
        impl < 'repo > References < 'repo > { # [doc = " Consumes a `References` iterator to create an iterator over just the"] # [doc = " name of some references."] # [doc = ""] # [doc = " This is more efficient if only the names are desired of references as"] # [doc = " the references themselves don't have to be allocated and deallocated."] # [doc = ""] # [doc = " The returned iterator will yield strings as opposed to a `Reference`."] pub fn names < 'a > (& 'a mut self) -> ReferenceNames < 'repo , 'a > { ReferenceNames { inner : self } } }
    };
}

impl_611!()