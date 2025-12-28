macro_rules! deps {
    () => {
        Ones!();
        FixedBitSet!();
    };
}

macro_rules! Difference {
    () => {
        deps!();
        # [doc = " An iterator producing elements in the difference of two sets."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::difference`] method."] pub struct Difference < 'a > { iter : Ones < 'a > , other : & 'a FixedBitSet , }
    };
}

Difference!()