macro_rules! IORING_ZCRX_AREA_MASK {
    () => {
        pub const IORING_ZCRX_AREA_MASK : u64 = ! ((1u64 << IORING_ZCRX_AREA_SHIFT) - 1) ;
    };
}

IORING_ZCRX_AREA_MASK!();