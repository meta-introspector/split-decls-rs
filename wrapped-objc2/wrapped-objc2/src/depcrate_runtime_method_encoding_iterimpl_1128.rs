// Generated macro for impl_1128 (impl)
macro_rules! Depcrate_runtime_method_encoding_iterimpl_1128 {
() => {
// Module: crate::runtime::method_encoding_iter
// Provides: {"impl_1128"}
// Dependencies: {}
impl < 'a > MethodEncodingIter < 'a > { pub (crate) fn new (s : & 'a str) -> Self { Self { s } } pub (crate) fn extract_return (& mut self ,) -> Result < (EncodingBox , Option < isize >) , EncodingParseError > { self . next () . ok_or (EncodingParseError :: MissingReturn) ? } pub (crate) fn verify_receiver (& mut self) -> Result < () , EncodingParseError > { let (enc , _stack_layout) = self . next () . ok_or (EncodingParseError :: MissingReceiver) ? ? ; if ! Encoding :: Object . equivalent_to_box (& enc) { return Err (EncodingParseError :: InvalidReceiver (enc)) ; } Ok (()) } pub (crate) fn verify_sel (& mut self) -> Result < () , EncodingParseError > { let (enc , _stack_layout) = self . next () . ok_or (EncodingParseError :: MissingSel) ? ? ; if ! Encoding :: Sel . equivalent_to_box (& enc) { return Err (EncodingParseError :: InvalidSel (enc)) ; } Ok (()) } fn extract_encoding (& mut self) -> Result < (EncodingBox , Option < isize >) , EncodingParseError > { let encoding = EncodingBox :: from_start_of_str (& mut self . s) ? ; let stack_layout = parse_stack_layout (& mut self . s) ? ; Ok ((encoding , stack_layout)) } }
};
}
