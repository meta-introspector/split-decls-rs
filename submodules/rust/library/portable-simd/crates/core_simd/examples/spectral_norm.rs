mkuse!{use core_simd :: simd :: prelude :: * ;}

macro_rules! a_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function a in module {}", module_path!());
    };
}

mkfn!{
    a_introspect!();
    fn a (i : usize , j : usize) -> f64 { ((i + j) * (i + j + 1) / 2 + i + 1) as f64 }
}

macro_rules! mult_av_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mult_av in module {}", module_path!());
    };
}

mkfn!{
    mult_av_introspect!();
    fn mult_av (v : & [f64] , out : & mut [f64]) { assert ! (v . len () == out . len ()) ; assert ! (v . len () % 2 == 0) ; for (i , out) in out . iter_mut () . enumerate () { let mut sum = f64x2 :: splat (0.0) ; let mut j = 0 ; while j < v . len () { let b = f64x2 :: from_slice (& v [j ..]) ; let a = f64x2 :: from_array ([a (i , j) , a (i , j + 1)]) ; sum += b / a ; j += 2 } * out = sum . reduce_sum () ; } }
}

macro_rules! mult_atv_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mult_atv in module {}", module_path!());
    };
}

mkfn!{
    mult_atv_introspect!();
    fn mult_atv (v : & [f64] , out : & mut [f64]) { assert ! (v . len () == out . len ()) ; assert ! (v . len () % 2 == 0) ; for (i , out) in out . iter_mut () . enumerate () { let mut sum = f64x2 :: splat (0.0) ; let mut j = 0 ; while j < v . len () { let b = f64x2 :: from_slice (& v [j ..]) ; let a = f64x2 :: from_array ([a (j , i) , a (j + 1 , i)]) ; sum += b / a ; j += 2 } * out = sum . reduce_sum () ; } }
}

macro_rules! mult_atav_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function mult_atav in module {}", module_path!());
    };
}

mkfn!{
    mult_atav_introspect!();
    fn mult_atav (v : & [f64] , out : & mut [f64] , tmp : & mut [f64]) { mult_av (v , tmp) ; mult_atv (tmp , out) ; }
}

macro_rules! spectral_norm_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function spectral_norm in module {}", module_path!());
    };
}

mkfn!{
    spectral_norm_introspect!();
    pub fn spectral_norm (n : usize) -> f64 { assert ! (n % 2 == 0 , "only even lengths are accepted") ; let mut u = vec ! [1.0 ; n] ; let mut v = u . clone () ; let mut tmp = u . clone () ; for _ in 0 .. 10 { mult_atav (& u , & mut v , & mut tmp) ; mult_atav (& v , & mut u , & mut tmp) ; } (dot (& u , & v) / dot (& v , & v)) . sqrt () }
}

macro_rules! dot_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot in module {}", module_path!());
    };
}

mkfn!{
    dot_introspect!();
    fn dot (x : & [f64] , y : & [f64]) -> f64 { x . iter () . zip (y) . map (| (& x , & y) | x * y) . sum () }
}

macro_rules! test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test in module {}", module_path!());
    };
}

mkfn!{
    test_introspect!();
    # [cfg (test)] # [test] fn test () { assert_eq ! (format ! ("{:.9}" , spectral_norm (100)) , "1.274219991") ; }
}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { }
}