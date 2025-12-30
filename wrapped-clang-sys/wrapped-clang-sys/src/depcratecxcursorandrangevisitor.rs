// Generated macro for CXCursorAndRangeVisitor (struct)
macro_rules! DepcrateCXCursorAndRangeVisitor {
() => {
// Module: crate
// Provides: {"CXCursorAndRangeVisitor"}
// Dependencies: {}
# [derive (Copy , Clone , Debug)] # [repr (C)] pub struct CXCursorAndRangeVisitor { pub context : * mut c_void , pub visit : Option < extern "C" fn (* mut c_void , CXCursor , CXSourceRange) -> CXVisitorResult > , }
};
}
