// Generated macro for impl_24 (impl)
macro_rules! Depcrate_typesimpl_24 {
() => {
// Module: crate::types
// Provides: {"impl_24"}
// Dependencies: {}
impl SubAssign < u32 > for Offset { fn sub_assign (& mut self , rhs : u32) { match self { Self :: Added (added) => { if rhs > * added { * self = Self :: Deleted (rhs - * added) ; } else { * self = Self :: Added (* added - rhs) ; } } Self :: Deleted (deleted) => * self = Self :: Deleted (* deleted + rhs) , } } }
};
}
