macro_rules! fence {
    () => {
        # [doc = " An atomic fence."] pub fn fence (order : Ordering) { crate :: rt :: fence (order) ; }
    };
}

fence!();