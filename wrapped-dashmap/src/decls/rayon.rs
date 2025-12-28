macro_rules! rayon {
    () => {
        # [cfg (feature = "rayon")] pub mod rayon { pub mod map ; pub mod read_only ; pub mod set ; }
    };
}

rayon!();