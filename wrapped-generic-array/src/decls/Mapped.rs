macro_rules! deps {
    () => {
        MappedGenericSequence!();
    };
}

macro_rules! Mapped {
    () => {
        deps!();
        # [doc = " Mapped type for a generic sequence"] pub type Mapped < S , T , U > = < S as MappedGenericSequence < T , U > > :: Mapped ;
    };
}

Mapped!()