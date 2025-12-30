// Generated macro for impl_1247 (impl)
macro_rules! Depcrate_rcimpl_1247 {
() => {
// Module: crate::rc
// Provides: {"impl_1247"}
// Dependencies: {}
# [doc = " We're doing this specialization here, and not as a more general optimization on `&T`, because it"] # [doc = " would otherwise add a cost to all equality checks on refs. We assume that `Rc`s are used to"] # [doc = " store large values, that are slow to clone, but also heavy to check for equality, causing this"] # [doc = " cost to pay off more easily. It's also more likely to have two `Rc` clones, that point to"] # [doc = " the same value, than two `&T`s."] # [doc = ""] # [doc = " We can only do this when `T: Eq` as a `PartialEq` might be deliberately irreflexive."] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + MarkerEq , A : Allocator > RcEqIdent < T , A > for Rc < T , A > { # [inline] fn eq (& self , other : & Rc < T , A >) -> bool { Rc :: ptr_eq (self , other) || * * self == * * other } # [inline] fn ne (& self , other : & Rc < T , A >) -> bool { ! Rc :: ptr_eq (self , other) && * * self != * * other } }
};
}
