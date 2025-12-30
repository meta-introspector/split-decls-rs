// Generated macro for impl_75 (impl)
macro_rules! Depcrate_errorimpl_75 {
() => {
// Module: crate::error
// Provides: {"impl_75"}
// Dependencies: {}
impl Error for CargoError { fn cause (& self) -> Option < & dyn Error > { self . cause . as_ref () . map (| c | { let c : & dyn Error = c . as_ref () ; c }) } }
};
}
