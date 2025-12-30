// Generated macro for tests (module)
macro_rules! Depcrate_provider_datatests {
() => {
// Module: crate::provider::data
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn test_roundtrip () { const TESTCASES : & [CaseMapData] = & [CaseMapData { ignoreable : true , kind : CaseMapDataKind :: Exception (Some (CaseType :: Title) , 923) , } , CaseMapData { ignoreable : false , kind : CaseMapDataKind :: Exception (None , 923) , } , CaseMapData { ignoreable : true , kind : CaseMapDataKind :: Delta (NonExceptionData { sensitive : true , dot_type : DotType :: SoftDotted , } , CaseType :: Upper , 50 ,) , } , CaseMapData { ignoreable : false , kind : CaseMapDataKind :: Delta (NonExceptionData { sensitive : true , dot_type : DotType :: SoftDotted , } , CaseType :: Upper , - 50 ,) , } , CaseMapData { ignoreable : false , kind : CaseMapDataKind :: Uncased (NonExceptionData { sensitive : false , dot_type : DotType :: SoftDotted , }) , } ,] ; for case in TESTCASES { let ule = case . to_unaligned () ; let roundtrip = CaseMapData :: from_unaligned (ule) ; assert_eq ! (* case , roundtrip) ; let integer = ule . 0 . as_unsigned_int () ; let roundtrip2 = CaseMapData :: try_from_icu_integer (integer) . unwrap () ; assert_eq ! (* case , roundtrip2) ; } } # [test] fn test_integer_roundtrip () { fn test_single_integer (int : u16) { let cmd = CaseMapData :: try_from_icu_integer (int) . unwrap () ; assert_eq ! (int , cmd . to_unaligned () . 0 . as_unsigned_int ()) } test_single_integer (84) ; test_single_integer (2503) ; } }
};
}
