// Generated macro for other_846 (other)
macro_rules! Depcrate_url_enumeratorother_846 {
() => {
// Module: crate::url_enumerator
// Provides: {"other_846"}
// Dependencies: {}
unsafe extern "C" { pub fn CFURLEnumeratorGetTypeID () -> CFTypeID ; pub fn CFURLEnumeratorCreateForDirectoryURL (alloc : CFAllocatorRef , directoryURL : CFURLRef , option : CFURLEnumeratorOptions , propertyKeys : CFArrayRef ,) -> CFURLEnumeratorRef ; pub fn CFURLEnumeratorCreateForMountedVolumes (alloc : CFAllocatorRef , option : CFURLEnumeratorOptions , propertyKeys : CFArrayRef ,) -> CFURLEnumeratorRef ; pub fn CFURLEnumeratorGetNextURL (enumerator : CFURLEnumeratorRef , url : * mut CFURLRef , error : * mut CFErrorRef ,) -> CFURLEnumeratorResult ; pub fn CFURLEnumeratorSkipDescendents (enumerator : CFURLEnumeratorRef) ; pub fn CFURLEnumeratorGetDescendentLevel (enumerator : CFURLEnumeratorRef) -> CFIndex ; pub fn CFURLEnumeratorGetSourceDidChange (enumerator : CFURLEnumeratorRef) -> Boolean ; }
};
}
