macro_rules! common {
    () => {
        fn common (ix : u32 , x : f32 , y1 : bool , sign : bool) -> f32 { let z : f64 ; let mut s : f64 ; let c : f64 ; let mut ss : f64 ; let mut cc : f64 ; s = sinf (x) as f64 ; if y1 { s = - s ; } c = cosf (x) as f64 ; cc = s - c ; if ix < 0x7f000000 { ss = - s - c ; z = cosf (2.0 * x) as f64 ; if s * c > 0.0 { cc = z / ss ; } else { ss = z / cc ; } if ix < 0x58800000 { if y1 { ss = - ss ; } cc = (ponef (x) as f64) * cc - (qonef (x) as f64) * ss ; } } if sign { cc = - cc ; } return (((INVSQRTPI as f64) * cc) / (sqrtf (x) as f64)) as f32 ; }
    };
}

common!();