macro_rules! MOVE_BLOCK_ALIGN {
    () => {
        # [doc = " Align to a 64-byte cache line"] const MOVE_BLOCK_ALIGN : i32 = 64 ;
    };
}

MOVE_BLOCK_ALIGN!();