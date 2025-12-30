// Generated macro for impl_223 (impl)
macro_rules! Depcrate_progress_keyimpl_223 {
() => {
// Module: crate::progress::key
// Provides: {"impl_223"}
// Dependencies: {}
impl SiblingLocation { fn merge (& mut self , other : SiblingLocation) { use SiblingLocation :: * ; * self = match (* self , other) { (any , NotFound) => any , (NotFound , any) => any , (Above , Below) => AboveAndBelow , (Below , Above) => AboveAndBelow , (AboveAndBelow , _) => AboveAndBelow , (_ , AboveAndBelow) => AboveAndBelow , (Above , Above) => Above , (Below , Below) => Below , } ; } }
};
}
