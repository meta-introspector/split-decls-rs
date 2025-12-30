// Generated macro for macro_550 (macro)
macro_rules! Depcrate_arbitrary__std_timemacro_550 {
() => {
// Module: crate::arbitrary::_std::time
// Provides: {"macro_550"}
// Dependencies: {}
arbitrary ! (SystemTime , statics :: Map < (num :: i32 :: Any , Range < u32 >) , fn ((i32 , u32)) -> SystemTime >; static_map ((num :: i32 :: ANY , 0 .. 1_000_000_000u32) , | (sec , ns) | { if sec >= 0 { UNIX_EPOCH + Duration :: new (sec as u64 , ns) } else { UNIX_EPOCH - Duration :: new ((- (sec as i64)) as u64 , ns) } })) ;
};
}
