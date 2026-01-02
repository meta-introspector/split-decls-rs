mkuse!{use super :: rem_pio2_large ;}
mkitem!{const EPS : f64 = 2.2204460492503131e-16 ;}
mkitem!{const TO_INT : f64 = 1.5 / EPS ;}
mkitem!{# [doc = " 53 bits of 2/pi"] const INV_PIO2 : f64 = 6.36619772367581382433e-01 ;}
mkitem!{# [doc = " first 33 bits of pi/2"] const PIO2_1 : f64 = 1.57079632673412561417e+00 ;}
mkitem!{# [doc = " pi/2 - PIO2_1"] const PIO2_1T : f64 = 6.07710050650619224932e-11 ;}
mkitem!{# [doc = " second 33 bits of pi/2"] const PIO2_2 : f64 = 6.07710050630396597660e-11 ;}
mkitem!{# [doc = " pi/2 - (PIO2_1+PIO2_2)"] const PIO2_2T : f64 = 2.02226624879595063154e-21 ;}
mkitem!{# [doc = " third 33 bits of pi/2"] const PIO2_3 : f64 = 2.02226624871116645580e-21 ;}
mkitem!{# [doc = " pi/2 - (PIO2_1+PIO2_2+PIO2_3)"] const PIO2_3T : f64 = 8.47842766036889956997e-32 ;}

macro_rules! rem_pio2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function rem_pio2 in module {}", module_path!());
    };
}

mkfn!{
    rem_pio2_introspect!();
    # [cfg_attr (assert_no_panic , no_panic :: no_panic)] pub (crate) fn rem_pio2 (x : f64) -> (i32 , f64 , f64) { let x1p24 = f64 :: from_bits (0x4170000000000000) ; let sign = (f64 :: to_bits (x) >> 63) as i32 ; let ix = (f64 :: to_bits (x) >> 32) as u32 & 0x7fffffff ; fn medium (x : f64 , ix : u32) -> (i32 , f64 , f64) { let tmp = x * INV_PIO2 + TO_INT ; # [cfg (all (target_arch = "x86" , not (target_feature = "sse2")))] let tmp = force_eval ! (tmp) ; let f_n = tmp - TO_INT ; let n = f_n as i32 ; let mut r = x - f_n * PIO2_1 ; let mut w = f_n * PIO2_1T ; let mut y0 = r - w ; let ui = f64 :: to_bits (y0) ; let ey = (ui >> 52) as i32 & 0x7ff ; let ex = (ix >> 20) as i32 ; if ex - ey > 16 { let t = r ; w = f_n * PIO2_2 ; r = t - w ; w = f_n * PIO2_2T - ((t - r) - w) ; y0 = r - w ; let ey = (f64 :: to_bits (y0) >> 52) as i32 & 0x7ff ; if ex - ey > 49 { let t = r ; w = f_n * PIO2_3 ; r = t - w ; w = f_n * PIO2_3T - ((t - r) - w) ; y0 = r - w ; } } let y1 = (r - y0) - w ; (n , y0 , y1) } if ix <= 0x400f6a7a { if (ix & 0xfffff) == 0x921fb { return medium (x , ix) ; } if ix <= 0x4002d97c { if sign == 0 { let z = x - PIO2_1 ; let y0 = z - PIO2_1T ; let y1 = (z - y0) - PIO2_1T ; return (1 , y0 , y1) ; } else { let z = x + PIO2_1 ; let y0 = z + PIO2_1T ; let y1 = (z - y0) + PIO2_1T ; return (- 1 , y0 , y1) ; } } else if sign == 0 { let z = x - 2.0 * PIO2_1 ; let y0 = z - 2.0 * PIO2_1T ; let y1 = (z - y0) - 2.0 * PIO2_1T ; return (2 , y0 , y1) ; } else { let z = x + 2.0 * PIO2_1 ; let y0 = z + 2.0 * PIO2_1T ; let y1 = (z - y0) + 2.0 * PIO2_1T ; return (- 2 , y0 , y1) ; } } if ix <= 0x401c463b { if ix <= 0x4015fdbc { if ix == 0x4012d97c { return medium (x , ix) ; } if sign == 0 { let z = x - 3.0 * PIO2_1 ; let y0 = z - 3.0 * PIO2_1T ; let y1 = (z - y0) - 3.0 * PIO2_1T ; return (3 , y0 , y1) ; } else { let z = x + 3.0 * PIO2_1 ; let y0 = z + 3.0 * PIO2_1T ; let y1 = (z - y0) + 3.0 * PIO2_1T ; return (- 3 , y0 , y1) ; } } else { if ix == 0x401921fb { return medium (x , ix) ; } if sign == 0 { let z = x - 4.0 * PIO2_1 ; let y0 = z - 4.0 * PIO2_1T ; let y1 = (z - y0) - 4.0 * PIO2_1T ; return (4 , y0 , y1) ; } else { let z = x + 4.0 * PIO2_1 ; let y0 = z + 4.0 * PIO2_1T ; let y1 = (z - y0) + 4.0 * PIO2_1T ; return (- 4 , y0 , y1) ; } } } if ix < 0x413921fb { return medium (x , ix) ; } if ix >= 0x7ff00000 { let y0 = x - x ; let y1 = y0 ; return (0 , y0 , y1) ; } let mut ui = f64 :: to_bits (x) ; ui &= (! 1) >> 12 ; ui |= (0x3ff + 23) << 52 ; let mut z = f64 :: from_bits (ui) ; let mut tx = [0.0 ; 3] ; for i in 0 .. 2 { i ! (tx , i , =, z as i32 as f64) ; z = (z - i ! (tx , i)) * x1p24 ; } i ! (tx , 2 , =, z) ; let mut i = 2 ; while i != 0 && i ! (tx , i) == 0.0 { i -= 1 ; } let mut ty = [0.0 ; 3] ; let n = rem_pio2_large (& tx [..= i] , & mut ty , ((ix as i32) >> 20) - (0x3ff + 23) , 1) ; if sign != 0 { return (- n , - i ! (ty , 0) , - i ! (ty , 1)) ; } (n , i ! (ty , 0) , i ! (ty , 1)) }
}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                mkuse!{use super :: rem_pio2 ;}

macro_rules! test_near_pi_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_near_pi in module {}", module_path!());
    };
}

mkfn!{
    test_near_pi_introspect!();
    # [test] # [cfg_attr (x86_no_sse , ignore)] fn test_near_pi () { let arg = 3.141592025756836 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , - 6.278329573009626e-7 , - 2.1125998133974653e-23)) ; let arg = 3.141592033207416 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , - 6.20382377148128e-7 , - 2.1125998133974653e-23)) ; let arg = 3.141592144966125 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , - 5.086236681942706e-7 , - 2.1125998133974653e-23)) ; let arg = 3.141592979431152 ; let arg = force_eval ! (arg) ; assert_eq ! (rem_pio2 (arg) , (2 , 3.2584135866119817e-7 , - 2.1125998133974653e-23)) ; }
}

macro_rules! test_overflow_b9b847_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_overflow_b9b847 in module {}", module_path!());
    };
}

mkfn!{
    test_overflow_b9b847_introspect!();
    # [test] fn test_overflow_b9b847 () { let _ = rem_pio2 (- 3054214.5490637687) ; }
}

macro_rules! test_overflow_4747b9_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test_overflow_4747b9 in module {}", module_path!());
    };
}

mkfn!{
    test_overflow_4747b9_introspect!();
    # [test] fn test_overflow_4747b9 () { let _ = rem_pio2 (917340800458.2274) ; }
} 
            }}