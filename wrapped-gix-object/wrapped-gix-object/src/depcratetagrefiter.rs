// Generated macro for TagRefIter (struct)
macro_rules! DepcrateTagRefIter {
() => {
// Module: crate
// Provides: {"TagRefIter"}
// Dependencies: {}
# [doc = " Like [`TagRef`], but as `Iterator` to support entirely allocation free parsing."] # [doc = " It's particularly useful to dereference only the target chain."] # [derive (Copy , Clone)] pub struct TagRefIter < 'a > { data : & 'a [u8] , state : tag :: ref_iter :: State , }
};
}
