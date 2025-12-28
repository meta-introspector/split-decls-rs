macro_rules! deps {
    () => {
        GeP2!();
        GeP3!();
        GeCached!();
        GeP1P1!();
    };
}

macro_rules! impl_106 {
    () => {
        deps!();
        impl GeP2 { fn zero () -> GeP2 { GeP2 { x : FE_ZERO , y : FE_ONE , z : FE_ONE , } } fn dbl (& self) -> GeP1P1 { let xx = self . x . square () ; let yy = self . y . square () ; let b = self . z . square_and_double () ; let a = self . x + self . y ; let aa = a . square () ; let y3 = yy + xx ; let z3 = yy - xx ; let x3 = aa - y3 ; let t3 = b - z3 ; GeP1P1 { x : x3 , y : y3 , z : z3 , t : t3 , } } fn slide (a : & [u8]) -> [i8 ; 256] { let mut r = [0i8 ; 256] ; for i in 0 .. 256 { r [i] = (1 & (a [i >> 3] >> (i & 7))) as i8 ; } for i in 0 .. 256 { if r [i] != 0 { for b in 1 .. min (7 , 256 - i) { if r [i + b] != 0 { if r [i] + (r [i + b] << b) <= 15 { r [i] += r [i + b] << b ; r [i + b] = 0 ; } else if r [i] - (r [i + b] << b) >= - 15 { r [i] -= r [i + b] << b ; for k in i + b .. 256 { if r [k] == 0 { r [k] = 1 ; break ; } r [k] = 0 ; } } else { break ; } } } } } r } # [allow (clippy :: comparison_chain)] pub fn double_scalarmult_vartime (a_scalar : & [u8] , a_point : GeP3 , b_scalar : & [u8]) -> GeP2 { let aslide = GeP2 :: slide (a_scalar) ; let bslide = GeP2 :: slide (b_scalar) ; let mut ai = [GeCached { y_plus_x : FE_ZERO , y_minus_x : FE_ZERO , z : FE_ZERO , t2d : FE_ZERO , } ; 8] ; ai [0] = a_point . to_cached () ; let a2 = a_point . dbl () . to_p3 () ; ai [1] = (a2 + ai [0]) . to_p3 () . to_cached () ; ai [2] = (a2 + ai [1]) . to_p3 () . to_cached () ; ai [3] = (a2 + ai [2]) . to_p3 () . to_cached () ; ai [4] = (a2 + ai [3]) . to_p3 () . to_cached () ; ai [5] = (a2 + ai [4]) . to_p3 () . to_cached () ; ai [6] = (a2 + ai [5]) . to_p3 () . to_cached () ; ai [7] = (a2 + ai [6]) . to_p3 () . to_cached () ; let mut r = GeP2 :: zero () ; let mut i : usize = 255 ; loop { if aslide [i] != 0 || bslide [i] != 0 { break ; } if i == 0 { return r ; } i -= 1 ; } loop { let mut t = r . dbl () ; if aslide [i] > 0 { t = t . to_p3 () + ai [(aslide [i] / 2) as usize] ; } else if aslide [i] < 0 { t = t . to_p3 () - ai [(- aslide [i] / 2) as usize] ; } if bslide [i] > 0 { t = t . to_p3 () + BI [(bslide [i] / 2) as usize] ; } else if bslide [i] < 0 { t = t . to_p3 () - BI [(- bslide [i] / 2) as usize] ; } r = t . to_p2 () ; if i == 0 { return r ; } i -= 1 ; } } }
    };
}

impl_106!();