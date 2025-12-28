macro_rules! deps {
    () => {
        Odd!();
    };
}

macro_rules! OddBoxedUint {
    () => {
        deps!();
        # [doc = " Non-zero boxed unsigned integer."] # [cfg (feature = "alloc")] pub type OddBoxedUint = Odd < BoxedUint > ;
    };
}

OddBoxedUint!();