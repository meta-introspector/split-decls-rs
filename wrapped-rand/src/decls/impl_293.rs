macro_rules! deps {
    () => {
        IndexedSamples!();
    };
}

macro_rules! impl_293 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl < 'a , S : Index < usize , Output = T > + ? Sized + 'a , T : 'a > ExactSizeIterator for IndexedSamples < 'a , S , T > { fn len (& self) -> usize { self . indices . len () } }
    };
}

impl_293!();