// Generated macro for impl_38 (impl)
macro_rules! Depcrate_arrayimpl_38 {
() => {
// Module: crate::array
// Provides: {"impl_38"}
// Dependencies: {}
impl < 'a , T : Type > Iterator for CFArrayIterUnchecked < 'a , T > { type Item = & 'a T ; # [inline] fn next (& mut self) -> Option < & 'a T > { debug_assert_eq ! (self . array . len () , self . len as usize , "array was mutated while iterating") ; if self . index < self . len { let value = unsafe { self . array . get_unchecked (self . index) } ; self . index += 1 ; Some (value) } else { None } } # [inline] fn size_hint (& self) -> (usize , Option < usize >) { let len = (self . len - self . index) as usize ; (len , Some (len)) } }
};
}
