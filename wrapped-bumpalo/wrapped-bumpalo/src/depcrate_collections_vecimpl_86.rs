// Generated macro for impl_86 (impl)
macro_rules! Depcrate_collections_vecimpl_86 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_86"}
// Dependencies: {}
# [cfg (feature = "boxed")] impl < 'bump , T > Vec < 'bump , T > { # [doc = " Converts the vector into [`Box<[T]>`][owned slice]."] # [doc = ""] # [doc = " Note that this will drop any excess capacity."] # [doc = ""] # [doc = " [owned slice]: ../../boxed/struct.Box.html"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use bumpalo::{Bump, collections::Vec, vec};"] # [doc = ""] # [doc = " let b = Bump::new();"] # [doc = ""] # [doc = " let v = vec![in &b; 1, 2, 3];"] # [doc = ""] # [doc = " let slice = v.into_boxed_slice();"] # [doc = " ```"] pub fn into_boxed_slice (mut self) -> crate :: boxed :: Box < 'bump , [T] > { use crate :: boxed :: Box ; unsafe { let slice = slice :: from_raw_parts_mut (self . as_mut_ptr () , self . len) ; let output : Box < 'bump , [T] > = Box :: from_raw (slice) ; mem :: forget (self) ; output } } }
};
}
