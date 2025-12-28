macro_rules! deps {
    () => {
        IndexedSamples!();
    };
}

macro_rules! impl_292 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a , S : Index < usize , Output = T > + ? Sized + 'a , T : 'a > Iterator for IndexedSamples < 'a , S , T > { type Item = & 'a T ; fn next (& mut self) -> Option < Self :: Item > { self . indices . next () . map (| i | & self . slice [i]) } fn size_hint (& self) -> (usize , Option < usize >) { (self . indices . len () , Some (self . indices . len ())) } }
    };
}

impl_292!();