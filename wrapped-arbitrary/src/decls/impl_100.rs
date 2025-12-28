macro_rules! deps {
    () => {
        Arbitrary!();
        Result!();
        Unstructured!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        # [doc = " Returns zero, not an error, if this `Unstructured` [is empty][Unstructured::is_empty]."] impl < 'a > Arbitrary < 'a > for AtomicIsize { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Arbitrary :: arbitrary (u) . map (Self :: new) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < isize as Arbitrary < 'a > > :: size_hint (depth) } }
    };
}

impl_100!();