macro_rules! MOVE_BLOCK_ALIGN_MASK {
    () => {
        const MOVE_BLOCK_ALIGN_MASK : i32 = ! (MOVE_BLOCK_ALIGN - 1) ;
    };
}

MOVE_BLOCK_ALIGN_MASK!()