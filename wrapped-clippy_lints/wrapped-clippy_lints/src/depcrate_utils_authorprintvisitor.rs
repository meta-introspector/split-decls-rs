// Generated macro for PrintVisitor (struct)
macro_rules! Depcrate_utils_authorPrintVisitor {
() => {
// Module: crate::utils::author
// Provides: {"PrintVisitor"}
// Dependencies: {}
struct PrintVisitor < 'a , 'tcx > { cx : & 'a LateContext < 'tcx > , # [doc = " Fields are the current index that needs to be appended to pattern"] # [doc = " binding names"] ids : Cell < FxHashMap < & 'static str , u32 > > , # [doc = " Currently at the first condition in the if chain"] first : Cell < bool > , }
};
}
