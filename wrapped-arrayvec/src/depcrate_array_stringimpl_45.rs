// Generated macro for impl_45 (impl)
macro_rules! Depcrate_array_stringimpl_45 {
() => {
// Module: crate::array_string
// Provides: {"impl_45"}
// Dependencies: {}
impl < const CAP : usize > PartialOrd < str > for ArrayString < CAP > { fn partial_cmp (& self , rhs : & str) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (rhs) } fn lt (& self , rhs : & str) -> bool { & * * self < rhs } fn le (& self , rhs : & str) -> bool { & * * self <= rhs } fn gt (& self , rhs : & str) -> bool { & * * self > rhs } fn ge (& self , rhs : & str) -> bool { & * * self >= rhs } }
};
}
