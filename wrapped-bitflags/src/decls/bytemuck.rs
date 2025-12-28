macro_rules! bytemuck {
    () => {
        # [cfg (feature = "bytemuck")] mod bytemuck ;
    };
}

bytemuck!();