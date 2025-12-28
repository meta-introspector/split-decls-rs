macro_rules! deps {
    () => {
        NonZero!();
    };
}

macro_rules! NonZeroBoxedUint {
    () => {
        deps!();
        # [doc = " Non-zero boxed unsigned integer."] # [cfg (feature = "alloc")] pub type NonZeroBoxedUint = NonZero < BoxedUint > ;
    };
}

NonZeroBoxedUint!()