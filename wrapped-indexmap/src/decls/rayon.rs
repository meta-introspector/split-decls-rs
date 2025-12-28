macro_rules! rayon {
    () => {
        # [cfg (feature = "rayon")] mod rayon ;
    };
}

rayon!();