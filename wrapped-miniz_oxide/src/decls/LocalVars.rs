macro_rules! deps {
    () => {
        BitBuffer!();
    };
}

macro_rules! LocalVars {
    () => {
        deps!();
        # [derive (Copy , Clone)] struct LocalVars { pub bit_buf : BitBuffer , pub num_bits : u32 , pub dist : u32 , pub counter : u32 , pub num_extra : u8 , }
    };
}

LocalVars!()