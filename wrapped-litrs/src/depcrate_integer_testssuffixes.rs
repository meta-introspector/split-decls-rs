// Generated macro for suffixes (function)
macro_rules! Depcrate_integer_testssuffixes {
() => {
// Module: crate::integer::tests
// Provides: {"suffixes"}
// Dependencies: {}
# [test] fn suffixes () { [("123i8" , Ty :: I8) , ("123i16" , Ty :: I16) , ("123i32" , Ty :: I32) , ("123i64" , Ty :: I64) , ("123i128" , Ty :: I128) , ("123u8" , Ty :: U8) , ("123u16" , Ty :: U16) , ("123u32" , Ty :: U32) , ("123u64" , Ty :: U64) , ("123u128" , Ty :: U128) ,] . iter () . for_each (| & (s , ty) | { assert_eq ! (Ty :: from_suffix (IntegerLit :: parse (s) . unwrap () . suffix ()) , Some (ty)) ; }) ; }
};
}
