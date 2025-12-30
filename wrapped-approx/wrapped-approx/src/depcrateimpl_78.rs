// Generated macro for impl_78 (impl)
macro_rules! Depcrateimpl_78 {
() => {
// Module: crate
// Provides: {"impl_78"}
// Dependencies: {}
impl < A , B > Relative < A , B > where A : RelativeEq < B > + ? Sized , B : ? Sized , { # [doc = " Replace the epsilon value with the one specified."] # [inline] pub fn epsilon (self , epsilon : A :: Epsilon) -> Relative < A , B > { Relative { epsilon , .. self } } # [doc = " Replace the maximum relative value with the one specified."] # [inline] pub fn max_relative (self , max_relative : A :: Epsilon) -> Relative < A , B > { Relative { max_relative , .. self } } # [doc = " Peform the equality comparison"] # [inline] # [must_use] pub fn eq (self , lhs : & A , rhs : & B) -> bool { A :: relative_eq (lhs , rhs , self . epsilon , self . max_relative) } # [doc = " Peform the inequality comparison"] # [inline] # [must_use] pub fn ne (self , lhs : & A , rhs : & B) -> bool { A :: relative_ne (lhs , rhs , self . epsilon , self . max_relative) } }
};
}
