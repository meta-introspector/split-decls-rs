// Generated macro for impl_189 (impl)
macro_rules! Depcrate_test_traitsimpl_189 {
() => {
// Module: crate::test_traits
// Provides: {"impl_189"}
// Dependencies: {}
impl < T1 , T2 , T3 > Hex for (T1 , T2 , T3) where T1 : Hex , T2 : Hex , T3 : Hex , { fn hex (self) -> String { format ! ("({}, {}, {})" , self . 0 . hex () , self . 1 . hex () , self . 2 . hex ()) } fn hexf (self) -> String { format ! ("({}, {}, {})" , self . 0 . hexf () , self . 1 . hexf () , self . 2 . hexf ()) } }
};
}
