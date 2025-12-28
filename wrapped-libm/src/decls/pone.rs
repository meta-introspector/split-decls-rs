macro_rules! pone {
    () => {
        fn pone (x : f64) -> f64 { let p : & [f64 ; 6] ; let q : & [f64 ; 5] ; let z : f64 ; let r : f64 ; let s : f64 ; let mut ix : u32 ; ix = get_high_word (x) ; ix &= 0x7fffffff ; if ix >= 0x40200000 { p = & PR8 ; q = & PS8 ; } else if ix >= 0x40122E8B { p = & PR5 ; q = & PS5 ; } else if ix >= 0x4006DB6D { p = & PR3 ; q = & PS3 ; } else { p = & PR2 ; q = & PS2 ; } z = 1.0 / (x * x) ; r = p [0] + z * (p [1] + z * (p [2] + z * (p [3] + z * (p [4] + z * p [5])))) ; s = 1.0 + z * (q [0] + z * (q [1] + z * (q [2] + z * (q [3] + z * q [4])))) ; return 1.0 + r / s ; }
    };
}

pone!();