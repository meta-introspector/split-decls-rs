// Generated macro for impl_389 (impl)
macro_rules! Depcrate_builder_value_parserimpl_389 {
() => {
// Module: crate::builder::value_parser
// Provides: {"impl_389"}
// Dependencies: {}
impl < T : TryFrom < i64 > + Clone + Send + Sync , B : RangeBounds < i64 > > From < B > for RangedI64ValueParser < T > { fn from (range : B) -> Self { Self { bounds : (range . start_bound () . cloned () , range . end_bound () . cloned ()) , target : Default :: default () , } } }
};
}
