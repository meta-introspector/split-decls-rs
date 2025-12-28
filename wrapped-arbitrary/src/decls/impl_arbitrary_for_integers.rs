macro_rules! deps {
    () => {
        Unstructured!();
        Arbitrary!();
        Result!();
    };
}

macro_rules! impl_arbitrary_for_integers {
    () => {
        deps!();
        macro_rules ! impl_arbitrary_for_integers { ($ ($ ty : ty ;) *) => { $ (impl <'a > Arbitrary <'a > for $ ty { fn arbitrary (u : & mut Unstructured <'a >) -> Result < Self > { let mut buf = [0 ; mem :: size_of ::<$ ty > ()] ; u . fill_buffer (& mut buf) ?; Ok (Self :: from_le_bytes (buf)) } # [inline] fn size_hint (_depth : usize) -> (usize , Option < usize >) { let n = mem :: size_of ::<$ ty > () ; (n , Some (n)) } }) * } }
    };
}

impl_arbitrary_for_integers!();