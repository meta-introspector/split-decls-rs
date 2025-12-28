macro_rules! deps {
    () => {
        Arbitrary!();
        Result!();
        Unstructured!();
    };
}

macro_rules! impl_104 {
    () => {
        deps!();
        # [doc = " Returns zero, not an error, if this `Unstructured` [is empty][Unstructured::is_empty]."] impl < 'a > Arbitrary < 'a > for Duration { fn arbitrary (u : & mut Unstructured < 'a >) -> Result < Self > { Ok (Self :: new (< u64 as Arbitrary > :: arbitrary (u) ? , u . int_in_range (0 ..= 999_999_999) ? ,)) } # [inline] fn size_hint (depth : usize) -> (usize , Option < usize >) { size_hint :: and (< u64 as Arbitrary > :: size_hint (depth) , < u32 as Arbitrary > :: size_hint (depth) ,) } }
    };
}

impl_104!();