// Generated macro for parse_overflowing_just_fine (function)
macro_rules! Depcrate_integer_testsparse_overflowing_just_fine {
() => {
// Module: crate::integer::tests
// Provides: {"parse_overflowing_just_fine"}
// Dependencies: {}
# [test] fn parse_overflowing_just_fine () { check ("256u8" , 256u16 , Decimal , "256" , Some (Ty :: U8)) ; check ("123_456_789u8" , 123_456_789u32 , Decimal , "123_456_789" , Some (Ty :: U8)) ; check ("123_456_789u16" , 123_456_789u32 , Decimal , "123_456_789" , Some (Ty :: U16)) ; check ("123_123_456_789u8" , 123_123_456_789u64 , Decimal , "123_123_456_789" , Some (Ty :: U8)) ; check ("123_123_456_789u16" , 123_123_456_789u64 , Decimal , "123_123_456_789" , Some (Ty :: U16)) ; check ("123_123_456_789u32" , 123_123_456_789u64 , Decimal , "123_123_456_789" , Some (Ty :: U32)) ; }
};
}
