// Generated macro for impl_44 (impl)
macro_rules! Depcrate_array_stringimpl_44 {
() => {
// Module: crate::array_string
// Provides: {"impl_44"}
// Dependencies: {}
impl < const CAP : usize > PartialOrd for ArrayString < CAP > { fn partial_cmp (& self , rhs : & Self) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (& * * rhs) } fn lt (& self , rhs : & Self) -> bool { * * self < * * rhs } fn le (& self , rhs : & Self) -> bool { * * self <= * * rhs } fn gt (& self , rhs : & Self) -> bool { * * self > * * rhs } fn ge (& self , rhs : & Self) -> bool { * * self >= * * rhs } }
};
}
