// Generated macro for impl_40 (impl)
macro_rules! Depcrate_isaac64impl_40 {
() => {
// Module: crate::isaac64
// Provides: {"impl_40"}
// Dependencies: {}
impl Isaac64Core { # [doc = " Create a new ISAAC-64 random number generator."] fn init (mut mem : [w64 ; RAND_SIZE] , rounds : u32) -> Self { # [rustfmt :: skip] fn mix (a : & mut w64 , b : & mut w64 , c : & mut w64 , d : & mut w64 , e : & mut w64 , f : & mut w64 , g : & mut w64 , h : & mut w64) { * a -= * e ; * f ^= * h >> 9 ; * h += * a ; * b -= * f ; * g ^= * a << 9 ; * a += * b ; * c -= * g ; * h ^= * b >> 23 ; * b += * c ; * d -= * h ; * a ^= * c << 15 ; * c += * d ; * e -= * a ; * b ^= * d >> 14 ; * d += * e ; * f -= * b ; * c ^= * e << 20 ; * e += * f ; * g -= * c ; * d ^= * f >> 17 ; * f += * g ; * h -= * d ; * e ^= * g << 14 ; * g += * h ; } let mut a = w (0x647c4677a2884b7c) ; let mut b = w (0xb9f8b322c73ac862) ; let mut c = w (0x8c0ea5053d4712a0) ; let mut d = w (0xb29b2e824a595524) ; let mut e = w (0x82f053db8355e0ce) ; let mut f = w (0x48fe4a0fa5a09315) ; let mut g = w (0xae985bf2cbfc89ed) ; let mut h = w (0x98f5704f6c44c0ab) ; for _ in 0 .. rounds { for i in (0 .. RAND_SIZE / 8) . map (| i | i * 8) { a += mem [i] ; b += mem [i + 1] ; c += mem [i + 2] ; d += mem [i + 3] ; e += mem [i + 4] ; f += mem [i + 5] ; g += mem [i + 6] ; h += mem [i + 7] ; mix (& mut a , & mut b , & mut c , & mut d , & mut e , & mut f , & mut g , & mut h ,) ; mem [i] = a ; mem [i + 1] = b ; mem [i + 2] = c ; mem [i + 3] = d ; mem [i + 4] = e ; mem [i + 5] = f ; mem [i + 6] = g ; mem [i + 7] = h ; } } Self { mem , a : w (0) , b : w (0) , c : w (0) , } } }
};
}
