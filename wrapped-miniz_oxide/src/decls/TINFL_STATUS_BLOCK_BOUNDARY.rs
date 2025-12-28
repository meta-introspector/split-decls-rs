macro_rules! TINFL_STATUS_BLOCK_BOUNDARY {
    () => {
        # [cfg (feature = "block-boundary")] const TINFL_STATUS_BLOCK_BOUNDARY : i32 = 3 ;
    };
}

TINFL_STATUS_BLOCK_BOUNDARY!()