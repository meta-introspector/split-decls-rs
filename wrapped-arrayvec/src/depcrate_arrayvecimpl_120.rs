// Generated macro for impl_120 (impl)
macro_rules! Depcrate_arrayvecimpl_120 {
() => {
// Module: crate::arrayvec
// Provides: {"impl_120"}
// Dependencies: {}
impl < T , const CAP : usize > PartialOrd for ArrayVec < T , CAP > where T : PartialOrd , { fn partial_cmp (& self , other : & Self) -> Option < cmp :: Ordering > { (* * self) . partial_cmp (other) } fn lt (& self , other : & Self) -> bool { (* * self) . lt (other) } fn le (& self , other : & Self) -> bool { (* * self) . le (other) } fn ge (& self , other : & Self) -> bool { (* * self) . ge (other) } fn gt (& self , other : & Self) -> bool { (* * self) . gt (other) } }
};
}
