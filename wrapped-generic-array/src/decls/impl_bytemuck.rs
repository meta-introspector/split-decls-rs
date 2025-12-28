macro_rules! impl_bytemuck {
    () => {
        # [cfg (feature = "bytemuck")] mod impl_bytemuck ;
    };
}

impl_bytemuck!();