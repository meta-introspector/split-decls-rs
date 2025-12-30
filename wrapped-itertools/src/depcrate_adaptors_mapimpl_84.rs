// Generated macro for impl_84 (impl)
macro_rules! Depcrate_adaptors_mapimpl_84 {
() => {
// Module: crate::adaptors::map
// Provides: {"impl_84"}
// Dependencies: {}
impl < F , T , U , E > MapSpecialCaseFn < Result < T , E > > for MapSpecialCaseFnOk < F > where F : FnMut (T) -> U , { type Out = Result < U , E > ; fn call (& mut self , t : Result < T , E >) -> Self :: Out { t . map (| v | self . 0 (v)) } }
};
}
