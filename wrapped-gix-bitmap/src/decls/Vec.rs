macro_rules! Vec {
    () => {
        # [doc = " A growable collection of u64 that are seen as stream of individual bits."] # [allow (dead_code)] # [derive (Clone)] pub struct Vec { num_bits : u32 , bits : std :: vec :: Vec < u64 > , # [doc = " RLW is an offset into the `bits` buffer, so `1` translates into &bits\\[1] essentially."] rlw : u64 , }
    };
}

Vec!();