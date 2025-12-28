macro_rules! NicheBias {
    () => {
        # [doc = " Determines towards which end of a struct layout optimizations will try to place the best niches."] enum NicheBias { Start , End , }
    };
}

NicheBias!();