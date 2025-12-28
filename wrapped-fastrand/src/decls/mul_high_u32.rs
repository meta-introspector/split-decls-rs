macro_rules! mul_high_u32 {
    () => {
        # [doc = " Computes `(a * b) >> 32`."] # [inline] fn mul_high_u32 (a : u32 , b : u32) -> u32 { (((a as u64) * (b as u64)) >> 32) as u32 }
    };
}

mul_high_u32!()