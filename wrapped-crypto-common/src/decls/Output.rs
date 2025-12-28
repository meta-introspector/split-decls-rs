macro_rules! deps {
    () => {
        OutputSizeUser!();
        OutputSize!();
    };
}

macro_rules! Output {
    () => {
        deps!();
        # [doc = " Output array of [`OutputSizeUser`] implementors."] pub type Output < T > = Array < u8 , OutputSize < T > > ;
    };
}

Output!();