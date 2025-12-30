// Generated macro for impl_39 (impl)
macro_rules! Depcrate_descriptorimpl_39 {
() => {
// Module: crate::descriptor
// Provides: {"impl_39"}
// Dependencies: {}
impl Function { fn decode (data : & mut & [u32]) -> Function { let shim_idx = get (data) ; let arguments = (0 .. get (data)) . map (| _ | Descriptor :: _decode (data , false)) . collect :: < Vec < _ > > () ; Function { arguments , shim_idx , ret : Descriptor :: _decode (data , false) , inner_ret : Some (Descriptor :: _decode (data , false)) , } } }
};
}
