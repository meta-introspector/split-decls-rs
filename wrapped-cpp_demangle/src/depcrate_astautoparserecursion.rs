// Generated macro for AutoParseRecursion (struct)
macro_rules! Depcrate_astAutoParseRecursion {
() => {
// Module: crate::ast
// Provides: {"AutoParseRecursion"}
// Dependencies: {}
# [doc = " An RAII type to automatically check the recursion level against the"] # [doc = " maximum. If the maximum has been crossed, return an error. Otherwise,"] # [doc = " increment the level upon construction, and decrement it upon destruction."] struct AutoParseRecursion < 'a > (& 'a ParseContext) ;
};
}
