// Generated macro for impl_1050 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1050 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1050"}
// Dependencies: {}
impl < S > GraphQLType < S > for SubscriptionRoot where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("SubscriptionRoot")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let fields = [] ; registry . build_object_type :: < Self > (i , & fields) . into_meta () } }
};
}
