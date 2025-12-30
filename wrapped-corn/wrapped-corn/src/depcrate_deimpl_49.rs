// Generated macro for impl_49 (impl)
macro_rules! Depcrate_deimpl_49 {
() => {
// Module: crate::de
// Provides: {"impl_49"}
// Dependencies: {}
impl < 'de > Seq < 'de > { fn new (value : Value < 'de >) -> Self { match value { Value :: Array (values) => Self { values : VecDeque :: from (values) , } , _ => unreachable ! () , } } }
};
}
