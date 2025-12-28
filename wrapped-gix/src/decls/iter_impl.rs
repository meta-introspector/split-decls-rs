macro_rules! deps {
    () => {
        Error!();
        Repository!();
        Item!();
        Info!();
    };
}

macro_rules! iter_impl {
    () => {
        deps!();
        pub (crate) mod iter_impl { # [doc = " The iterator returned by [`crate::revision::walk::Platform::all()`]."] pub struct Walk < 'repo > { # [doc = " The owning repository."] pub repo : & 'repo crate :: Repository , pub (crate) inner : Box < dyn Iterator < Item = Result < gix_traverse :: commit :: Info , super :: iter :: Error > > + 'repo > , } impl < 'repo > Iterator for Walk < 'repo > { type Item = Result < super :: Info < 'repo > , super :: iter :: Error > ; fn next (& mut self) -> Option < Self :: Item > { self . inner . next () . map (| res | res . map (| info | super :: Info :: new (info , self . repo))) } } }
    };
}

iter_impl!();