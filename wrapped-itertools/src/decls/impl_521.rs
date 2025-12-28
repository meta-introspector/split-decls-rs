macro_rules! deps {
    () => {
        TupleCollect!();
        CircularTupleWindows!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl < I , T > Iterator for CircularTupleWindows < I , T > where I : Iterator < Item = T :: Item > + Clone , T : TupleCollect + Clone , T :: Item : Clone , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . len != 0 { self . len -= 1 ; self . iter . next () } else { None } } fn size_hint (& self) -> (usize , Option < usize >) { (self . len , Some (self . len)) } }
    };
}

impl_521!()