mkuse!{use super :: { k_cos , k_sin , rem_pio2 } ;}

macro_rules! sin_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function sin in module {}", module_path!());
    };
}

mkfn!{
    sin_introspect!();
    # [doc = " The sine of `x` (f64)."] # [doc = ""] # [doc = " `x` is specified in radians."] # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub fn sin (x : f64) -> f64 { let x1p120 = f64 :: from_bits (0x4770000000000000) ; let ix = (f64 :: to_bits (x) >> 32) as u32 & 0x7fffffff ; if ix <= 0x3fe921fb { if ix < 0x3e500000 { if ix < 0x00100000 { force_eval ! (x / x1p120) ; } else { force_eval ! (x + x1p120) ; } return x ; } return k_sin (x , 0.0 , 0) ; } if ix >= 0x7ff00000 { return x - x ; } let (n , y0 , y1) = rem_pio2 (x) ; match n & 3 { 0 => k_sin (y0 , y1 , 1) , 1 => k_cos (y0 , y1) , 2 => - k_sin (y0 , y1 , 1) , _ => - k_cos (y0 , y1) , } }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: * ;}

macro_rules! test_near_pi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_near_pi in module {}", module_path!());
    };
}

mkfn!{
    test_near_pi_introspect!();
    # [test] # [cfg_attr (x86_no_sse , ignore = "FIXME(i586): possible incorrect rounding")] fn test_near_pi () { let x = f64 :: from_bits (0x400921fb000FD5DD) ; let sx = f64 :: from_bits (0x3ea50d15ced1a4a2) ; assert_eq ! (sin (x) , sx) ; }
} 
            }}