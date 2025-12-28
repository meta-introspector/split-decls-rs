macro_rules! deps {
    () => {
        Row!();
        Data!();
        Scale!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl < A , B , C > Row for (A , B , C) where A : Data , B : Data , C : Data , { type Scale = (f64 , f64 , f64) ; fn append_to (self , buffer : & mut Vec < u8 > , scale : (f64 , f64 , f64)) { let (a , b , c) = self ; write_f64 (buffer , a . f64 () * scale . 0) . unwrap () ; write_f64 (buffer , b . f64 () * scale . 1) . unwrap () ; write_f64 (buffer , c . f64 () * scale . 2) . unwrap () ; } fn ncols () -> usize { 3 } }
    };
}

impl_7!()