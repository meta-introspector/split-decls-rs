// Generated macro for CFDictionaryRetainCallBack (type)
macro_rules! Depcrate_dictionaryCFDictionaryRetainCallBack {
() => {
// Module: crate::dictionary
// Provides: {"CFDictionaryRetainCallBack"}
// Dependencies: {}
pub type CFDictionaryRetainCallBack = extern "C" fn (allocator : CFAllocatorRef , value : * const c_void) -> * const c_void ;
};
}
