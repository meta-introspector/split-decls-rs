macro_rules! deps {
    () => {
        Lcg128Xsl64!();
    };
}

macro_rules! Pcg64 {
    () => {
        deps!();
        # [doc = " [`Lcg128Xsl64`] is also officially known as `pcg64`."] pub type Pcg64 = Lcg128Xsl64 ;
    };
}

Pcg64!();