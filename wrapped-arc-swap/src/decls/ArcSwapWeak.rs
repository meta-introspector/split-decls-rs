macro_rules! deps {
    () => {
        ArcSwap!();
        ArcSwapAny!();
    };
}

macro_rules! ArcSwapWeak {
    () => {
        deps!();
        # [doc = " Arc swap for the [Weak] pointer."] # [doc = ""] # [doc = " This is similar to [ArcSwap], but it doesn't store [Arc], it stores [Weak]. It doesn't keep the"] # [doc = " data alive when pointed to."] # [doc = ""] # [doc = " This is a type alias only. Most of the methods are described on the"] # [doc = " [`ArcSwapAny`](struct.ArcSwapAny.html)."] # [doc = ""] # [doc = " Needs the `weak` feature turned on."] # [doc = ""] # [doc = " [Weak]: std::sync::Weak"] # [cfg (feature = "weak")] pub type ArcSwapWeak < T > = ArcSwapAny < alloc :: sync :: Weak < T > > ;
    };
}

ArcSwapWeak!()