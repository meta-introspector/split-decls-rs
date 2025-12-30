// Generated macro for impl_46 (impl)
macro_rules! Depcrate_deimpl_46 {
() => {
// Module: crate::de
// Provides: {"impl_46"}
// Dependencies: {}
impl < 'de > Map < 'de > { fn new (value : Value < 'de >) -> Self { match value { Value :: Object (values) => Self { values : values . into_iter () . flat_map (| (key , value) | vec ! [Value :: String (key) , value]) . collect () , } , _ => unreachable ! () , } } }
};
}
