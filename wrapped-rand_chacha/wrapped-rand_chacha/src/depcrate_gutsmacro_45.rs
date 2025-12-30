// Generated macro for macro_45 (macro)
macro_rules! Depcrate_gutsmacro_45 {
() => {
// Module: crate::guts
// Provides: {"macro_45"}
// Dependencies: {}
dispatch ! (m , Mach , { fn refill_narrow_rounds (state : & mut ChaCha , drounds : u32) -> State < vec128_storage > { let k : Mach :: u32x4 = m . vec ([0x6170_7865 , 0x3320_646e , 0x7962_2d32 , 0x6b20_6574]) ; let mut x = State { a : k , b : m . unpack (state . b) , c : m . unpack (state . c) , d : m . unpack (state . d) , } ; for _ in 0 .. drounds { x = round (x) ; x = undiagonalize (round (diagonalize (x))) ; } State { a : x . a . into () , b : x . b . into () , c : x . c . into () , d : x . d . into () , } } }) ;
};
}
