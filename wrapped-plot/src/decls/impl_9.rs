macro_rules! deps {
    () => {
        Row!();
        Scale!();
        Data!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < A , B , C , D , E > Row for (A , B , C , D , E) where A : Data , B : Data , C : Data , D : Data , E : Data , { type Scale = (f64 , f64 , f64 , f64 , f64) ; # [allow (clippy :: many_single_char_names)] fn append_to (self , buffer : & mut Vec < u8 > , scale : (f64 , f64 , f64 , f64 , f64)) { let (a , b , c , d , e) = self ; write_f64 (buffer , a . f64 () * scale . 0) . unwrap () ; write_f64 (buffer , b . f64 () * scale . 1) . unwrap () ; write_f64 (buffer , c . f64 () * scale . 2) . unwrap () ; write_f64 (buffer , d . f64 () * scale . 3) . unwrap () ; write_f64 (buffer , e . f64 () * scale . 4) . unwrap () ; } fn ncols () -> usize { 5 } }
    };
}

impl_9!();