macro_rules! qonef {
    () => {
        fn qonef (x : f32) -> f32 { let p : & [f32 ; 6] ; let q : & [f32 ; 6] ; let s : f32 ; let r : f32 ; let z : f32 ; let mut ix : u32 ; ix = x . to_bits () ; ix &= 0x7fffffff ; if ix >= 0x41000000 { p = & QR8 ; q = & QS8 ; } else if ix >= 0x409173eb { p = & QR5 ; q = & QS5 ; } else if ix >= 0x4036d917 { p = & QR3 ; q = & QS3 ; } else { p = & QR2 ; q = & QS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * (q [4] + z * q [5]))))) ; return (0.375 + r / s) / x ; }
    };
}

qonef!()