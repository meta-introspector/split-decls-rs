// Generated macro for primitives (macro)
macro_rules! Depcrate_runtime_typeprimitives {
() => {
// Module: crate::runtime_type
// Provides: {"primitives"}
// Dependencies: {}
macro_rules ! primitives { ($ (($ t : ty , $ s : literal)) ,+) => { $ (impl RuntimeType for $ t { const SIGNATURE : imp :: ConstBuffer = imp :: ConstBuffer :: from_slice ($ s) ; }) * } ; }
};
}
