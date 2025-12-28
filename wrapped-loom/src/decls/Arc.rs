macro_rules! Arc {
    () => {
        # [doc = " Mock implementation of `std::sync::Arc`."] # [derive (Debug)] pub struct Arc < T : ? Sized > { obj : std :: sync :: Arc < rt :: Arc > , value : std :: sync :: Arc < T > , }
    };
}

Arc!();