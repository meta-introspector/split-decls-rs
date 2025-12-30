// Generated macro for impl_406 (impl)
macro_rules! Depcrate_proto_streams_prioritizeimpl_406 {
() => {
// Module: crate::proto::streams::prioritize
// Provides: {"impl_406"}
// Dependencies: {}
impl < B : Buf > fmt :: Debug for Prioritized < B > { fn fmt (& self , fmt : & mut fmt :: Formatter) -> fmt :: Result { fmt . debug_struct ("Prioritized") . field ("remaining" , & self . inner . get_ref () . remaining ()) . field ("end_of_stream" , & self . end_of_stream) . field ("stream" , & self . stream) . finish () } }
};
}
