macro_rules! deps {
    () => {
        BlockBoundaryState!();
    };
}

macro_rules! impl_140 {
    () => {
        deps!();
        # [cfg (feature = "block-boundary")] impl Default for BlockBoundaryState { fn default () -> Self { BlockBoundaryState { num_bits : 0 , bit_buf : 0 , z_header0 : 0 , z_header1 : 0 , check_adler32 : 1 , } } }
    };
}

impl_140!();