// Generated macro for marshal_impl (macro)
macro_rules! Depcrate_marshalledmarshal_impl {
() => {
// Module: crate::marshalled
// Provides: {"marshal_impl"}
// Dependencies: {}
macro_rules ! marshal_impl { ($ t : ty , $ s : expr , $ a : expr) => { impl Marshal for $ t { fn signature (& self) -> & SignatureSingle { SignatureSingle :: new_unchecked ($ s) } fn append_data_to (& self , v : & mut Vec < u8 >) { align_buf (v , $ a) ; v . extend_from_slice (& self . to_ne_bytes ()) } } } }
};
}
