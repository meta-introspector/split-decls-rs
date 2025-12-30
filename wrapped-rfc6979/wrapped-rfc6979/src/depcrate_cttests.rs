// Generated macro for tests (module)
macro_rules! Depcrate_cttests {
() => {
// Module: crate::ct
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { const A : [u8 ; 4] = [0 , 0 , 0 , 0] ; const B : [u8 ; 4] = [0 , 0 , 0 , 1] ; const C : [u8 ; 4] = [0xFF , 0 , 0 , 0] ; const D : [u8 ; 4] = [0xFF , 0 , 0 , 1] ; const E : [u8 ; 4] = [0xFF , 0xFF , 0xFF , 0xFE] ; const F : [u8 ; 4] = [0xFF , 0xFF , 0xFF , 0xFF] ; # [test] fn ct_is_zero () { use super :: is_zero ; assert_eq ! (is_zero (& A) . unwrap_u8 () , 1) ; assert_eq ! (is_zero (& B) . unwrap_u8 () , 0) ; } # [test] fn ct_lt () { use super :: lt ; assert_eq ! (lt (& A , & A) . unwrap_u8 () , 0) ; assert_eq ! (lt (& B , & B) . unwrap_u8 () , 0) ; assert_eq ! (lt (& C , & C) . unwrap_u8 () , 0) ; assert_eq ! (lt (& D , & D) . unwrap_u8 () , 0) ; assert_eq ! (lt (& E , & E) . unwrap_u8 () , 0) ; assert_eq ! (lt (& F , & F) . unwrap_u8 () , 0) ; assert_eq ! (lt (& A , & B) . unwrap_u8 () , 1) ; assert_eq ! (lt (& A , & C) . unwrap_u8 () , 1) ; assert_eq ! (lt (& B , & A) . unwrap_u8 () , 0) ; assert_eq ! (lt (& C , & A) . unwrap_u8 () , 0) ; assert_eq ! (lt (& B , & C) . unwrap_u8 () , 1) ; assert_eq ! (lt (& B , & D) . unwrap_u8 () , 1) ; assert_eq ! (lt (& C , & B) . unwrap_u8 () , 0) ; assert_eq ! (lt (& D , & B) . unwrap_u8 () , 0) ; assert_eq ! (lt (& C , & D) . unwrap_u8 () , 1) ; assert_eq ! (lt (& C , & E) . unwrap_u8 () , 1) ; assert_eq ! (lt (& D , & C) . unwrap_u8 () , 0) ; assert_eq ! (lt (& E , & C) . unwrap_u8 () , 0) ; assert_eq ! (lt (& E , & F) . unwrap_u8 () , 1) ; assert_eq ! (lt (& F , & E) . unwrap_u8 () , 0) ; } }
};
}
