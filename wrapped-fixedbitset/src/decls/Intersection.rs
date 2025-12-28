macro_rules! deps {
    () => {
        Ones!();
        FixedBitSet!();
    };
}

macro_rules! Intersection {
    () => {
        deps!();
        # [doc = " An iterator producing elements in the intersection of two sets."] # [doc = ""] # [doc = " This struct is created by the [`FixedBitSet::intersection`] method."] pub struct Intersection < 'a > { iter : Ones < 'a > , other : & 'a FixedBitSet , }
    };
}

Intersection!()