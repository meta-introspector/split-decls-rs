macro_rules! deps {
    () => {
        ArcSwapAny!();
    };
}

macro_rules! ArcSwap {
    () => {
        deps!();
        # [doc = " An atomic storage for `Arc`."] # [doc = ""] # [doc = " This is a type alias only. Most of its methods are described on"] # [doc = " [`ArcSwapAny`](struct.ArcSwapAny.html)."] pub type ArcSwap < T > = ArcSwapAny < Arc < T > > ;
    };
}

ArcSwap!()