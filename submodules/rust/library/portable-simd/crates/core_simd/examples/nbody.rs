mkitem!{extern crate std_float ;}
mkmod!{nbody, { 
                getname!(nbody);
                getsrc!(nbody);
                getpath!(nbody);
                get_deps!(nbody);
                get_crates!(nbody);
                mkinclude!(nbody);
                mkuse!{use core_simd :: simd :: prelude :: * ;}
mkuse!{# [allow (unused)] use std_float :: StdFloat ;}
mkuse!{use std :: f64 :: consts :: PI ;}
mkitem!{const SOLAR_MASS : f64 = 4.0 * PI * PI ;}
mkitem!{const DAYS_PER_YEAR : f64 = 365.24 ;}
mkitem!{mkstruct!{# [derive (Debug , Clone , Copy)] struct Body { pub x : f64x4 , pub v : f64x4 , pub mass : f64 , }}}
mkitem!{const N_BODIES : usize = 5 ;}
mkitem!{const BODIES : [Body ; N_BODIES] = [Body { x : f64x4 :: from_array ([0. , 0. , 0. , 0.]) , v : f64x4 :: from_array ([0. , 0. , 0. , 0.]) , mass : SOLAR_MASS , } , Body { x : f64x4 :: from_array ([4.84143144246472090e+00 , - 1.16032004402742839e+00 , - 1.03622044471123109e-01 , 0. ,]) , v : f64x4 :: from_array ([1.66007664274403694e-03 * DAYS_PER_YEAR , 7.69901118419740425e-03 * DAYS_PER_YEAR , - 6.90460016972063023e-05 * DAYS_PER_YEAR , 0. ,]) , mass : 9.54791938424326609e-04 * SOLAR_MASS , } , Body { x : f64x4 :: from_array ([8.34336671824457987e+00 , 4.12479856412430479e+00 , - 4.03523417114321381e-01 , 0. ,]) , v : f64x4 :: from_array ([- 2.76742510726862411e-03 * DAYS_PER_YEAR , 4.99852801234917238e-03 * DAYS_PER_YEAR , 2.30417297573763929e-05 * DAYS_PER_YEAR , 0. ,]) , mass : 2.85885980666130812e-04 * SOLAR_MASS , } , Body { x : f64x4 :: from_array ([1.28943695621391310e+01 , - 1.51111514016986312e+01 , - 2.23307578892655734e-01 , 0. ,]) , v : f64x4 :: from_array ([2.96460137564761618e-03 * DAYS_PER_YEAR , 2.37847173959480950e-03 * DAYS_PER_YEAR , - 2.96589568540237556e-05 * DAYS_PER_YEAR , 0. ,]) , mass : 4.36624404335156298e-05 * SOLAR_MASS , } , Body { x : f64x4 :: from_array ([1.53796971148509165e+01 , - 2.59193146099879641e+01 , 1.79258772950371181e-01 , 0. ,]) , v : f64x4 :: from_array ([2.68067772490389322e-03 * DAYS_PER_YEAR , 1.62824170038242295e-03 * DAYS_PER_YEAR , - 9.51592254519715870e-05 * DAYS_PER_YEAR , 0. ,]) , mass : 5.15138902046611451e-05 * SOLAR_MASS , } ,] ;}

macro_rules! offset_momentum_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function offset_momentum in module {}", module_path!());
    };
}

mkfn!{
    offset_momentum_introspect!();
    fn offset_momentum (bodies : & mut [Body ; N_BODIES]) { let (sun , rest) = bodies . split_at_mut (1) ; let sun = & mut sun [0] ; for body in rest { let m_ratio = body . mass / SOLAR_MASS ; sun . v -= body . v * Simd :: splat (m_ratio) ; } }
}

macro_rules! energy_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function energy in module {}", module_path!());
    };
}

mkfn!{
    energy_introspect!();
    fn energy (bodies : & [Body ; N_BODIES]) -> f64 { let mut e = 0. ; for i in 0 .. N_BODIES { let bi = & bodies [i] ; e += bi . mass * (bi . v * bi . v) . reduce_sum () * 0.5 ; for bj in bodies . iter () . take (N_BODIES) . skip (i + 1) { let dx = bi . x - bj . x ; e -= bi . mass * bj . mass / (dx * dx) . reduce_sum () . sqrt () } } e }
}

macro_rules! advance_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function advance in module {}", module_path!());
    };
}

mkfn!{
    advance_introspect!();
    fn advance (bodies : & mut [Body ; N_BODIES] , dt : f64) { const N : usize = N_BODIES * (N_BODIES - 1) / 2 ; let mut r = [f64x4 :: splat (0.) ; N] ; { let mut i = 0 ; for j in 0 .. N_BODIES { for k in j + 1 .. N_BODIES { r [i] = bodies [j] . x - bodies [k] . x ; i += 1 ; } } } let mut mag = [0.0 ; N] ; for i in (0 .. N) . step_by (2) { let d2s = f64x2 :: from_array ([(r [i] * r [i]) . reduce_sum () , (r [i + 1] * r [i + 1]) . reduce_sum () ,]) ; let dmags = f64x2 :: splat (dt) / (d2s * d2s . sqrt ()) ; mag [i] = dmags [0] ; mag [i + 1] = dmags [1] ; } let mut i = 0 ; for j in 0 .. N_BODIES { for k in j + 1 .. N_BODIES { let f = r [i] * Simd :: splat (mag [i]) ; bodies [j] . v -= f * Simd :: splat (bodies [k] . mass) ; bodies [k] . v += f * Simd :: splat (bodies [j] . mass) ; i += 1 } } for body in bodies { body . x += Simd :: splat (dt) * body . v } }
}

macro_rules! run_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function run in module {}", module_path!());
    };
}

mkfn!{
    run_introspect!();
    pub fn run (n : usize) -> (f64 , f64) { let mut bodies = BODIES ; offset_momentum (& mut bodies) ; let energy_before = energy (& bodies) ; for _ in 0 .. n { advance (& mut bodies , 0.01) ; } let energy_after = energy (& bodies) ; (energy_before , energy_after) }
} 
            }}
mkmod!{tests, { 
                getname!(tests);
                getsrc!(tests);
                getpath!(tests);
                get_deps!(tests);
                get_crates!(tests);
                mkinclude!(tests);
                
macro_rules! approx_eq_f64_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function approx_eq_f64 in module {}", module_path!());
    };
}

mkfn!{
    approx_eq_f64_introspect!();
    fn approx_eq_f64 (a : f64 , b : f64) -> bool { (a - b) . abs () < 0.00001 }
}

macro_rules! test_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function test in module {}", module_path!());
    };
}

mkfn!{
    test_introspect!();
    # [test] fn test () { const OUTPUT : [f64 ; 2] = [- 0.169075164 , - 0.169087605] ; let (energy_before , energy_after) = super :: nbody :: run (1000) ; assert ! (approx_eq_f64 (energy_before , OUTPUT [0])) ; assert ! (approx_eq_f64 (energy_after , OUTPUT [1])) ; }
} 
            }}

macro_rules! main_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function main in module {}", module_path!());
    };
}

mkfn!{
    main_introspect!();
    fn main () { { let (energy_before , energy_after) = nbody :: run (1000) ; println ! ("Energy before: {energy_before}") ; println ! ("Energy after:  {energy_after}") ; } }
}