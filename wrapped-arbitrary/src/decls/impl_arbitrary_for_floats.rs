macro_rules! deps {
    () => {
        Arbitrary!();
        Result!();
        Unstructured!();
    };
}

macro_rules! impl_arbitrary_for_floats {
    () => {
        deps!();
        macro_rules ! impl_arbitrary_for_floats { ($ ($ ty : ident : $ unsigned : ty ;) *) => { $ (impl <'a > Arbitrary <'a > for $ ty { fn arbitrary (u : & mut Unstructured <'a >) -> Result < Self > { Ok (Self :: from_bits (<$ unsigned as Arbitrary <'a >>:: arbitrary (u) ?)) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { <$ unsigned as Arbitrary <'a >>:: size_hint (depth) } }) * } }
    };
}

impl_arbitrary_for_floats!();