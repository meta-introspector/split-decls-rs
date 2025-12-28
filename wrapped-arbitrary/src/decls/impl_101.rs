macro_rules! deps {
    () => {
        Unstructured!();
        Arbitrary!();
        Result!();
    };
}

macro_rules! impl_101 {
    () => {
        deps!();
        # [doc = " Returns zero, not an error, if this `Unstructured` [is empty][Unstructured::is_empty]."] impl < 'a > Arbitrary < 'a > for AtomicUsize { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < usize as Arbitrary < 'a > > :: size_hint (depth) } }
    };
}

impl_101!();