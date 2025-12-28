macro_rules! deps {
    () => {
        Mcg128Xsl64!();
    };
}

macro_rules! Pcg64Mcg {
    () => {
        deps!();
        # [doc = " A friendly name for [`Mcg128Xsl64`] (also known as `pcg64_fast`)."] pub type Pcg64Mcg = Mcg128Xsl64 ;
    };
}

Pcg64Mcg!();