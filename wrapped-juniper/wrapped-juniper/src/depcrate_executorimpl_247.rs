// Generated macro for impl_247 (impl)
macro_rules! Depcrate_executorimpl_247 {
() => {
// Module: crate::executor
// Provides: {"impl_247"}
// Dependencies: {}
impl < S > Ord for ExecutionError < S > where Self : Eq , { fn cmp (& self , other : & ExecutionError < S >) -> Ordering { (& self . location , & self . path , & self . error . message) . cmp (& (& other . location , & other . path , & other . error . message ,)) } }
};
}
