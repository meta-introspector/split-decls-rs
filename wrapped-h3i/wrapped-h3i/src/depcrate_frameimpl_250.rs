// Generated macro for impl_250 (impl)
macro_rules! Depcrate_frameimpl_250 {
() => {
// Module: crate::frame
// Provides: {"impl_250"}
// Dependencies: {}
impl Serialize for Comparator { fn serialize < S > (& self , serializer : S) -> Result < S :: Ok , S :: Error > where S : Serializer , { match self { Self :: Fn (_) => serializer . serialize_str ("<comparator_fn>") , Self :: Frame (f) => { let mut frame_ser = serializer . serialize_struct ("frame" , 1) ? ; frame_ser . serialize_field ("frame" , f) ? ; frame_ser . end () } , } } }
};
}
