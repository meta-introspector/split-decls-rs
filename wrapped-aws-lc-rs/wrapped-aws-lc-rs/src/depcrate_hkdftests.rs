// Generated macro for tests (module)
macro_rules! Depcrate_hkdftests {
() => {
// Module: crate::hkdf
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use crate :: hkdf :: { Salt , HKDF_SHA256 , HKDF_SHA384 } ; # [cfg (feature = "fips")] mod fips ; # [test] fn hkdf_coverage () { assert_ne ! (HKDF_SHA256 , HKDF_SHA384) ; assert_eq ! ("Algorithm(Algorithm(SHA256))" , format ! ("{HKDF_SHA256:?}")) ; } # [test] fn test_debug () { const SALT : & [u8 ; 32] = & [29 , 113 , 120 , 243 , 11 , 202 , 39 , 222 , 206 , 81 , 163 , 184 , 122 , 153 , 52 , 192 , 98 , 195 , 240 , 32 , 34 , 19 , 160 , 128 , 178 , 111 , 97 , 232 , 113 , 101 , 221 , 143 ,] ; const SECRET1 : & [u8 ; 32] = & [157 , 191 , 36 , 107 , 110 , 131 , 193 , 6 , 175 , 226 , 193 , 3 , 168 , 133 , 165 , 181 , 65 , 120 , 194 , 152 , 31 , 92 , 37 , 191 , 73 , 222 , 41 , 112 , 207 , 236 , 196 , 174 ,] ; const INFO1 : & [& [u8]] = & [& [2 , 130 , 61 , 83 , 192 , 248 , 63 , 60 , 211 , 73 , 169 , 66 , 101 , 160 , 196 , 212 , 250 , 113 ,] , & [80 , 46 , 248 , 123 , 78 , 204 , 171 , 178 , 67 , 204 , 96 , 27 , 131 , 24 ,] ,] ; let alg = HKDF_SHA256 ; let salt = Salt :: new (alg , SALT) ; let prk = salt . extract (SECRET1) ; let okm = prk . expand (INFO1 , alg) . unwrap () ; assert_eq ! ("hkdf::Salt { algorithm: Algorithm(SHA256) }" , format ! ("{salt:?}")) ; assert_eq ! ("hkdf::Prk { algorithm: Algorithm(SHA256), mode: ExtractExpand { .. } }" , format ! ("{prk:?}")) ; assert_eq ! ("hkdf::Okm { prk: hkdf::Prk { algorithm: Algorithm(SHA256), mode: ExtractExpand { .. } } }" , format ! ("{okm:?}")) ; } }
};
}
