macro_rules! deps {
    () => {
        Unstructured!();
        Result!();
        Arbitrary!();
    };
}

macro_rules! impl_23 {
    () => {
        deps!();
        impl < 'a > Arbitrary < 'a > for CString { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { < Vec < u8 > as Arbitrary > :: arbitrary (u) . map (| mut x | { x . retain (| & c | c != 0) ; unsafe { Self :: from_vec_unchecked (x) } }) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { < Vec < u8 > as Arbitrary > :: size_hint (depth) } }
    };
}

impl_23!();