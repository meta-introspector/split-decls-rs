// Generated macro for impl_13 (impl)
macro_rules! Depcrate_compressorimpl_13 {
() => {
// Module: crate::compressor
// Provides: {"impl_13"}
// Dependencies: {}
impl < M : Machine > X8 < M > { # [inline (always)] fn zip (self) -> (M :: u128x2 , M :: u128x2 , M :: u128x2 , M :: u128x2) { ([self . 0 , self . 1] . vzip () , [self . 2 , self . 3] . vzip () , [self . 4 , self . 5] . vzip () , [self . 6 , self . 7] . vzip () ,) } # [inline (always)] fn unzip ((a , b , c , d) : (M :: u128x2 , M :: u128x2 , M :: u128x2 , M :: u128x2)) -> Self { X8 (a . extract (0) , a . extract (1) , b . extract (0) , b . extract (1) , c . extract (0) , c . extract (1) , d . extract (0) , d . extract (1) ,) } }
};
}
