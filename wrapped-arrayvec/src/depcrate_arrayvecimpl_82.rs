// Generated macro for impl_82 (impl)
macro_rules! Depcrate_arrayvecimpl_82 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_82"}
// Dependencies: {}
# [doc = " Try to create an `ArrayVec` from a slice. This will return an error if the slice was too big to"] # [doc = " fit."] # [doc = ""] # [doc = " ```"] # [doc = " use arrayvec::ArrayVec;"] # [doc = " use std::convert::TryInto as _;"] # [doc = ""] # [doc = " let array: ArrayVec<_, 4> = (&[1, 2, 3] as &[_]).try_into().unwrap();"] # [doc = " assert_eq!(array.len(), 3);"] # [doc = " assert_eq!(array.capacity(), 4);"] # [doc = " ```"] impl < T , const CAP : usize > std :: convert :: TryFrom < & [T] > for ArrayVec < T , CAP > where T : Clone , { type Error = CapacityError ; fn try_from (slice : & [T]) -> Result < Self , Self :: Error > { if Self :: CAPACITY < slice . len () { Err (CapacityError :: new (())) } else { let mut array = Self :: new () ; array . extend_from_slice (slice) ; Ok (array) } } }
};
}
