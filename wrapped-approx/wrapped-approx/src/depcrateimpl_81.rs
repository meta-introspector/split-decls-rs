// Generated macro for impl_81 (impl)
macro_rules! Depcrateimpl_81 {
() => {
// Module: crate
// Provides: {"impl_81"}
// Dependencies: {}
impl < A , B > Ulps < A , B > where A : UlpsEq < B > + ? Sized , B : ? Sized , { # [doc = " Replace the epsilon value with the one specified."] # [inline] pub fn epsilon (self , epsilon : A :: Epsilon) -> Ulps < A , B > { Ulps { epsilon , .. self } } # [doc = " Replace the max ulps value with the one specified."] # [inline] pub fn max_ulps (self , max_ulps : u32) -> Ulps < A , B > { Ulps { max_ulps , .. self } } # [doc = " Peform the equality comparison"] # [inline] # [must_use] pub fn eq (self , lhs : & A , rhs : & B) -> bool { A :: ulps_eq (lhs , rhs , self . epsilon , self . max_ulps) } # [doc = " Peform the inequality comparison"] # [inline] # [must_use] pub fn ne (self , lhs : & A , rhs : & B) -> bool { A :: ulps_ne (lhs , rhs , self . epsilon , self . max_ulps) } }
};
}
