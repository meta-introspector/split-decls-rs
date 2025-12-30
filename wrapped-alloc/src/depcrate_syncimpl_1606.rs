// Generated macro for impl_1606 (impl)
macro_rules! Depcrate_syncimpl_1606 {
() => {
// Module: crate::sync
// Provides: {"impl_1606"}
// Dependencies: {}
# [doc = " We're doing this specialization here, and not as a more general optimization on `&T`, because it"] # [doc = " would otherwise add a cost to all equality checks on refs. We assume that `Arc`s are used to"] # [doc = " store large values, that are slow to clone, but also heavy to check for equality, causing this"] # [doc = " cost to pay off more easily. It's also more likely to have two `Arc` clones, that point to"] # [doc = " the same value, than two `&T`s."] # [doc = ""] # [doc = " We can only do this when `T: Eq` as a `PartialEq` might be deliberately irreflexive."] # [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized + crate :: rc :: MarkerEq , A : Allocator > ArcEqIdent < T , A > for Arc < T , A > { # [inline] fn eq (& self , other : & Arc < T , A >) -> bool { Arc :: ptr_eq (self , other) || * * self == * * other } # [inline] fn ne (& self , other : & Arc < T , A >) -> bool { ! Arc :: ptr_eq (self , other) && * * self != * * other } }
};
}
