// Generated macro for _STATIC_ASSERT_TAG_END_SIZE (const)
macro_rules! Depcrate_STATIC_ASSERT_TAG_END_SIZE {
() => {
// Module: crate
// Provides: {"_STATIC_ASSERT_TAG_END_SIZE"}
// Dependencies: {}
# [doc = " Make sure `TagEnd` is no more than two bytes in size."] # [doc = " This is why it's used instead of just using `Tag`."] # [cfg (target_pointer_width = "64")] const _STATIC_ASSERT_TAG_END_SIZE : [() ; 2] = [() ; core :: mem :: size_of :: < TagEnd > ()] ;
};
}
