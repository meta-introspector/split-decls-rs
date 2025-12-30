// Generated macro for Marshal (trait)
macro_rules! Depcrate_typesMarshal {
() => {
// Module: crate::types
// Provides: {"Marshal"}
// Dependencies: {}
pub trait Marshal { const ALIGN : usize ; fn signature () -> Cow < 'static , SignatureSingle > ; fn write_buf < B : Write + Seek > (& self , _ : & mut MarshalState < B >) -> IoResult < () > ; }
};
}
