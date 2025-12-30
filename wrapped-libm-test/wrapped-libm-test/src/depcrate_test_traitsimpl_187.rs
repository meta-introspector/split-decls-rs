// Generated macro for impl_187 (impl)
macro_rules! Depcrate_test_traitsimpl_187 {
() => {
// Module: crate::test_traits
// Provides: {"impl_187"}
// Dependencies: {}
impl < T1 , T2 > Hex for (T1 , T2) where T1 : Hex , T2 : Hex , { fn hex (self) -> String { format ! ("({}, {})" , self . 0 . hex () , self . 1 . hex ()) } fn hexf (self) -> String { format ! ("({}, {})" , self . 0 . hexf () , self . 1 . hexf ()) } }
};
}
