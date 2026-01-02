mkuse!{use core_simd :: simd :: prelude :: * ;}

macro_rules! dot_prod_scalar_0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_scalar_0 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_scalar_0_introspect!();
    pub fn dot_prod_scalar_0 (a : & [f32] , b : & [f32]) -> f32 { assert_eq ! (a . len () , b . len ()) ; a . iter () . zip (b . iter ()) . map (| (a , b) | a * b) . sum () }
}

macro_rules! dot_prod_scalar_1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_scalar_1 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_scalar_1_introspect!();
    pub fn dot_prod_scalar_1 (a : & [f32] , b : & [f32]) -> f32 { assert_eq ! (a . len () , b . len ()) ; a . iter () . zip (b . iter ()) . fold (0.0 , | a , zipped | a + zipped . 0 * zipped . 1) }
}

macro_rules! dot_prod_simd_0_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_simd_0 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_simd_0_introspect!();
    pub fn dot_prod_simd_0 (a : & [f32] , b : & [f32]) -> f32 { assert_eq ! (a . len () , b . len ()) ; a . array_chunks :: < 4 > () . map (| & a | f32x4 :: from_array (a)) . zip (b . array_chunks :: < 4 > () . map (| & b | f32x4 :: from_array (b))) . map (| (a , b) | (a * b) . reduce_sum ()) . sum () }
}

macro_rules! dot_prod_simd_1_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_simd_1 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_simd_1_introspect!();
    pub fn dot_prod_simd_1 (a : & [f32] , b : & [f32]) -> f32 { assert_eq ! (a . len () , b . len ()) ; a . array_chunks :: < 4 > () . map (| & a | f32x4 :: from_array (a)) . zip (b . array_chunks :: < 4 > () . map (| & b | f32x4 :: from_array (b))) . fold (f32x4 :: splat (0.0) , | acc , zipped | acc + zipped . 0 * zipped . 1) . reduce_sum () }
}
mkuse!{use std_float :: StdFloat ;}

macro_rules! dot_prod_simd_2_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_simd_2 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_simd_2_introspect!();
    pub fn dot_prod_simd_2 (a : & [f32] , b : & [f32]) -> f32 { assert_eq ! (a . len () , b . len ()) ; let mut res = f32x4 :: splat (0.0) ; a . array_chunks :: < 4 > () . map (| & a | f32x4 :: from_array (a)) . zip (b . array_chunks :: < 4 > () . map (| & b | f32x4 :: from_array (b))) . for_each (| (a , b) | { res = a . mul_add (b , res) ; }) ; res . reduce_sum () }
}
mkitem!{const LANES : usize = 4 ;}

macro_rules! dot_prod_simd_3_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_simd_3 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_simd_3_introspect!();
    pub fn dot_prod_simd_3 (a : & [f32] , b : & [f32]) -> f32 { assert_eq ! (a . len () , b . len ()) ; let (a_extra , a_chunks) = a . as_rchunks () ; let (b_extra , b_chunks) = b . as_rchunks () ; assert_eq ! (a_chunks . len () , b_chunks . len ()) ; assert_eq ! (a_extra . len () , b_extra . len ()) ; let mut sums = [0.0 ; LANES] ; for ((x , y) , d) in std :: iter :: zip (a_extra , b_extra) . zip (& mut sums) { * d = x * y ; } let mut sums = f32x4 :: from_array (sums) ; std :: iter :: zip (a_chunks , b_chunks) . for_each (| (x , y) | { sums += f32x4 :: from_array (* x) * f32x4 :: from_array (* y) ; }) ; sums . reduce_sum () }
}

macro_rules! dot_prod_simd_4_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_simd_4 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_simd_4_introspect!();
    pub fn dot_prod_simd_4 (a : & [f32] , b : & [f32]) -> f32 { let mut sum = a . array_chunks :: < 4 > () . map (| & a | f32x4 :: from_array (a)) . zip (b . array_chunks :: < 4 > () . map (| & b | f32x4 :: from_array (b))) . map (| (a , b) | a * b) . fold (f32x4 :: splat (0.0) , std :: ops :: Add :: add) . reduce_sum () ; let remain = a . len () - (a . len () % 4) ; sum += a [remain ..] . iter () . zip (& b [remain ..]) . map (| (a , b) | a * b) . sum :: < f32 > () ; sum }
}

macro_rules! dot_prod_simd_5_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function dot_prod_simd_5 in module {}", module_path!());
    };
}

mkfn!{
    dot_prod_simd_5_introspect!();
    pub fn dot_prod_simd_5 (a : & [f32] , b : & [f32]) -> f32 { a . array_chunks :: < 4 > () . map (| & a | f32x4 :: from_array (a)) . zip (b . array_chunks :: < 4 > () . map (| & b | f32x4 :: from_array (b))) . fold (f32x4 :: splat (0.) , | acc , (a , b) | a . mul_add (b , acc)) . reduce_sum () }
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
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                
macro_rules! smoke_test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function smoke_test in module {}", module_path!());
    };
}

mkfn!{
    smoke_test_introspect!();
    # [test] fn smoke_test () { use super :: * ; let a : Vec < f32 > = vec ! [1.0 , 2.0 , 3.0 , 4.0 , 5.0 , 6.0 , 7.0 , 8.0] ; let b : Vec < f32 > = vec ! [- 8.0 , - 7.0 , - 6.0 , - 5.0 , 4.0 , 3.0 , 2.0 , 1.0] ; let x : Vec < f32 > = [0.5 ; 1003] . to_vec () ; let y : Vec < f32 > = [2.0 ; 1003] . to_vec () ; assert_eq ! (0.0 , dot_prod_scalar_0 (& a , & b)) ; assert_eq ! (0.0 , dot_prod_scalar_1 (& a , & b)) ; assert_eq ! (0.0 , dot_prod_simd_0 (& a , & b)) ; assert_eq ! (0.0 , dot_prod_simd_1 (& a , & b)) ; assert_eq ! (0.0 , dot_prod_simd_2 (& a , & b)) ; assert_eq ! (0.0 , dot_prod_simd_3 (& a , & b)) ; assert_eq ! (0.0 , dot_prod_simd_4 (& a , & b)) ; assert_eq ! (0.0 , dot_prod_simd_5 (& a , & b)) ; assert_eq ! (1003.0 , dot_prod_simd_3 (& x , & y)) ; }
} 
            }}