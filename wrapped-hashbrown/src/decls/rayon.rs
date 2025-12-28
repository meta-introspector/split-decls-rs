macro_rules! rayon {
    () => {
        # [cfg (feature = "rayon")] pub (crate) mod rayon ;
    };
}

rayon!();