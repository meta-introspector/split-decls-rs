macro_rules! deps {
    () => {
        ArcSwap!();
        ArcSwapAny!();
    };
}

macro_rules! ArcSwapOption {
    () => {
        deps!();
        # [doc = " An atomic storage for `Option<Arc>`."] # [doc = ""] # [doc = " This is very similar to [`ArcSwap`](type.ArcSwap.html), but allows storing NULL values, which"] # [doc = " is useful in some situations."] # [doc = ""] # [doc = " This is a type alias only. Most of the methods are described on"] # [doc = " [`ArcSwapAny`](struct.ArcSwapAny.html). Even though the examples there often use `ArcSwap`,"] # [doc = " they are applicable to `ArcSwapOption` with appropriate changes."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::Arc;"] # [doc = " use arc_swap::ArcSwapOption;"] # [doc = ""] # [doc = " let shared = ArcSwapOption::from(None);"] # [doc = " assert!(shared.load_full().is_none());"] # [doc = " assert!(shared.swap(Some(Arc::new(42))).is_none());"] # [doc = " assert_eq!(42, **shared.load_full().as_ref().unwrap());"] # [doc = " ```"] pub type ArcSwapOption < T > = ArcSwapAny < Option < Arc < T > > > ;
    };
}

ArcSwapOption!()