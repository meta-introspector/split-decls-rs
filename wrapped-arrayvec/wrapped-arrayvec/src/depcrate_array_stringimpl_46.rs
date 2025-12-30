// Generated macro for impl_46 (impl)
macro_rules! Depcrate_array_stringimpl_46 {
() => {
// Module: crate::array_string
// Provides: {"impl_46"}
// Dependencies: {}
impl < const CAP : usize > PartialOrd < ArrayString < CAP > > for str { fn partial_cmp (& self , rhs : & ArrayString < CAP >) -> Option < cmp :: Ordering > { self . partial_cmp (& * * rhs) } fn lt (& self , rhs : & ArrayString < CAP >) -> bool { self < & * * rhs } fn le (& self , rhs : & ArrayString < CAP >) -> bool { self <= & * * rhs } fn gt (& self , rhs : & ArrayString < CAP >) -> bool { self > & * * rhs } fn ge (& self , rhs : & ArrayString < CAP >) -> bool { self >= & * * rhs } }
};
}
