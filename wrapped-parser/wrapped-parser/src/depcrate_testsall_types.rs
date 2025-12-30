// Generated macro for all_types (function)
macro_rules! Depcrate_testsall_types {
() => {
// Module: crate::tests
// Provides: {"all_types"}
// Dependencies: {}
# [rstest] # [case ("=i8" , Type :: I8)] # [case ("=i16" , Type :: I16)] # [case ("=i32" , Type :: I32)] # [case ("=i64" , Type :: I64)] # [case ("=i128" , Type :: I128)] # [case ("=isize" , Type :: Isize)] # [case ("=u8" , Type :: U8)] # [case ("=u16" , Type :: U16)] # [case ("=u32" , Type :: U32)] # [case ("=u64" , Type :: U64)] # [case ("=u128" , Type :: U128)] # [case ("=usize" , Type :: Usize)] # [case ("=f32" , Type :: F32)] # [case ("=f64" , Type :: F64)] # [case ("=bool" , Type :: Bool)] # [case ("=?" , Type :: Format)] # [case ("=str" , Type :: Str)] # [case ("=[u8]" , Type :: U8Slice)] fn all_types (# [case] input : & str , # [case] ty : Type) { assert_eq ! (parse_param (input , ParserMode :: Strict) , Ok (Param { index : None , ty , hint : None , })) ; }
};
}
