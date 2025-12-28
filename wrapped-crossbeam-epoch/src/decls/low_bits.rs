macro_rules! deps {
    () => {
        Pointable!();
    };
}

macro_rules! low_bits {
    () => {
        deps!();
        # [doc = " Returns a bitmask containing the unused least significant bits of an aligned pointer to `T`."] # [inline] fn low_bits < T : ? Sized + Pointable > () -> usize { (1 << T :: ALIGN . trailing_zeros ()) - 1 }
    };
}

low_bits!()