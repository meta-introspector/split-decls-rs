macro_rules! cautious {
    () => {
        # [cfg (feature = "serde")] pub fn cautious (hint : Option < usize >) -> usize { cmp :: min (hint . unwrap_or (0) , 4096) }
    };
}

cautious!();