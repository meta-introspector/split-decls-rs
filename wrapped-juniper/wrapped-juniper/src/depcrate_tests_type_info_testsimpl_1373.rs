// Generated macro for impl_1373 (impl)
macro_rules! Depcrate_tests_type_info_testsimpl_1373 {
() => {
// Module: crate::tests::type_info_tests
// Provides: {"impl_1373"}
// Dependencies: {}
impl < S > GraphQLType < S > for Node where S : ScalarValue , { fn name (info : & Self :: TypeInfo) -> Option < ArcStr > { Some (info . name . clone ()) } fn meta (info : & Self :: TypeInfo , registry : & mut Registry < S >) -> MetaType < S > { let fields = info . attribute_names . iter () . map (| name | registry . field :: < String > (name . clone () , & ())) . collect :: < Vec < _ > > () ; registry . build_object_type :: < Node > (info , & fields) . into_meta () } }
};
}
