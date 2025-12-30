// Generated macro for impl_1048 (impl)
macro_rules! Depcrate_validation_test_harnessimpl_1048 {
() => {
// Module: crate::validation::test_harness
// Provides: {"impl_1048"}
// Dependencies: {}
impl < S > GraphQLType < S > for MutationRoot where S : ScalarValue , { fn name (_ : & ()) -> Option < ArcStr > { Some (arcstr :: literal ! ("MutationRoot")) } fn meta (i : & () , registry : & mut Registry < S >) -> MetaType < S > { let _ = registry . get_type :: < Unpopulated > (i) ; let fields = [registry . field :: < i32 > (arcstr :: literal ! ("testInput") , i) . argument (registry . arg_with_default :: < TestInput > (arcstr :: literal ! ("input") , & TestInput { id : 423 , name : String :: from ("foo") , } , i ,))] ; registry . build_object_type :: < Self > (i , & fields) . into_meta () } }
};
}
