macro_rules! IdxRange {
    () => {
        # [doc = " A range of densely allocated arena values."] pub struct IdxRange < T > { range : Range < u32 > , _p : PhantomData < T > , }
    };
}

IdxRange!();