macro_rules! deps {
    () => {
        Resize!();
        Odd!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl Resize for Odd < BoxedUint > { type Output = Self ; fn resize_unchecked (self , at_least_bits_precision : u32) -> Self :: Output { Odd (self . 0 . resize_unchecked (at_least_bits_precision)) } fn try_resize (self , at_least_bits_precision : u32) -> Option < Self :: Output > { self . 0 . try_resize (at_least_bits_precision) . map (Odd) } }
    };
}

impl_244!()