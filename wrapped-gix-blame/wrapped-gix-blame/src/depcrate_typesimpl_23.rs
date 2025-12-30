// Generated macro for impl_23 (impl)
macro_rules! Depcrate_typesimpl_23 {
() => {
// Module: crate::types
// Provides: {"impl_23"}
// Dependencies: {}
impl AddAssign < u32 > for Offset { fn add_assign (& mut self , rhs : u32) { match self { Self :: Added (added) => * self = Self :: Added (* added + rhs) , Self :: Deleted (deleted) => { if rhs > * deleted { * self = Self :: Added (rhs - * deleted) ; } else { * self = Self :: Deleted (* deleted - rhs) ; } } } } }
};
}
