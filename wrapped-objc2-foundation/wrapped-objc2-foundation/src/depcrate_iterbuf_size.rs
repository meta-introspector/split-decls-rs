// Generated macro for BUF_SIZE (const)
macro_rules! Depcrate_iterBUF_SIZE {
() => {
// Module: crate::iter
// Provides: {"BUF_SIZE"}
// Dependencies: {}
# [doc = " Swift and Objective-C both have a stack buffer size of 16, so we do that"] # [doc = " as well."] # [doc = ""] # [doc = " TODO: Consider lowering this a bit, since the most common type of"] # [doc = " enumeration (e.g. NSArray) doesn't use the buffer:"] # [doc = " [CFArray's NSFastEnumeration implementation](https://github.com/apple-oss-distributions/CF/blob/dc54c6bb1c1e5e0b9486c1d26dd5bef110b20bf3/CFArray.c#L618-L642)"] const BUF_SIZE : usize = 16 ;
};
}
