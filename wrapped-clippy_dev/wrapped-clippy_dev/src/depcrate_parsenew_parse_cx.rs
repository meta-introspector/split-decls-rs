// Generated macro for new_parse_cx (function)
macro_rules! Depcrate_parsenew_parse_cx {
() => {
// Module: crate::parse
// Provides: {"new_parse_cx"}
// Dependencies: {}
# [doc = " Calls the given function inside a newly created parsing context."] pub fn new_parse_cx < 'env , T > (f : impl for < 'cx > FnOnce (& 'cx mut Scoped < 'cx , 'env , ParseCxImpl < 'cx > >) -> T) -> T { let arena = DroplessArena :: default () ; f (& mut Scoped :: new (ParseCxImpl { arena : & arena , str_buf : StrBuf :: with_capacity (128) , })) }
};
}
