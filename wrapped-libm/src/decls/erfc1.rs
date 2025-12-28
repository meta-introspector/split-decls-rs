macro_rules! erfc1 {
    () => {
        fn erfc1 (x : f32) -> f32 { let s : f32 ; let p : f32 ; let q : f32 ; s = fabsf (x) - 1.0 ; p = PA0 + s * (PA1 + s * (PA2 + s * (PA3 + s * (PA4 + s * (PA5 + s * PA6))))) ; q = 1.0 + s * (QA1 + s * (QA2 + s * (QA3 + s * (QA4 + s * (QA5 + s * QA6))))) ; return 1.0 - ERX - p / q ; }
    };
}

erfc1!()