// Generated macro for impl_97 (impl)
macro_rules! Depcrate_marshalledimpl_97 {
() => {
// Module: crate::marshalled
// Provides: {"impl_97"}
// Dependencies: {}
impl VariantBuf { pub fn new < T : Marshal + ? Sized > (value : & T) -> Result < Self , DemarshalError > { let mut data = vec ! () ; value . append_data_to (& mut data) ; Ok (VariantBuf { sig : value . signature () . into () , data }) } }
};
}
