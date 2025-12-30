// Generated macro for macro_450 (macro)
macro_rules! Depcrate_arbitrary__std_iomacro_450 {
() => {
// Module: crate::arbitrary::_std::io
// Provides: {"macro_450"}
// Dependencies: {}
arbitrary ! (Error , SMapped < (ErrorKind , Option < String >) , Self >; static_map (arbitrary () , | (k , os) | if let Some (s) = os { Error :: new (k , s) } else { k . into () })) ;
};
}
