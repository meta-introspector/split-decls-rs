macro_rules! deps {
    () => {
        Unsigned!();
        Limb!();
    };
}

macro_rules! Word {
    () => {
        deps!();
        # [doc = " Unsigned integer type that the [`Limb`] newtype wraps."] # [cfg (target_pointer_width = "64")] pub type Word = u64 ;
    };
}

Word!()