macro_rules! deps {
    () => {
        ARM64_NT_NEON128_0!();
    };
}

macro_rules! other_98 {
    () => {
        deps!();
        # [repr (C)] # [derive (Clone , Copy)] pub union ARM64_NT_NEON128 { pub Anonymous : ARM64_NT_NEON128_0 , pub D : [f64 ; 2] , pub S : [f32 ; 4] , pub H : [u16 ; 8] , pub B : [u8 ; 16] , }
    };
}

other_98!();