// Generated macro for TagForeachCB (type)
macro_rules! Depcrate_tagforeachTagForeachCB {
() => {
// Module: crate::tagforeach
// Provides: {"TagForeachCB"}
// Dependencies: {}
# [doc = " boxed callback type"] pub (crate) type TagForeachCB < 'a > = Box < dyn FnMut (Oid , & [u8]) -> bool + 'a > ;
};
}
