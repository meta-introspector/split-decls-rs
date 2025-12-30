// Generated macro for impl_394 (impl)
macro_rules! Depcrate_builder_value_parserimpl_394 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_394"}
// Dependencies: {}
impl < T : TryFrom < u64 > , B : RangeBounds < u64 > > From < B > for RangedU64ValueParser < T > { fn from (range : B) -> Self { Self { bounds : (range . start_bound () . cloned () , range . end_bound () . cloned ()) , target : Default :: default () , } } }
};
}
