// Generated macro for impl_75 (impl)
macro_rules! Depcrateimpl_75 {
() => {
// Module: crate
// Provides: {"impl_75"}
// Dependencies: {}
impl < A , B > AbsDiff < A , B > where A : AbsDiffEq < B > + ? Sized , B : ? Sized , { # [doc = " Replace the epsilon value with the one specified."] # [inline] pub fn epsilon (self , epsilon : A :: Epsilon) -> AbsDiff < A , B > { AbsDiff { epsilon } } # [doc = " Peform the equality comparison"] # [inline] # [must_use] pub fn eq (self , lhs : & A , rhs : & B) -> bool { A :: abs_diff_eq (lhs , rhs , self . epsilon) } # [doc = " Peform the inequality comparison"] # [inline] # [must_use] pub fn ne (self , lhs : & A , rhs : & B) -> bool { A :: abs_diff_ne (lhs , rhs , self . epsilon) } }
};
}
