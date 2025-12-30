// Generated macro for impl_24 (impl)
macro_rules! Depcrate_driverimpl_24 {
() => {
// Module: crate::driver
// Provides: {"impl_24"}
// Dependencies: {}
impl < Sink : TreeSink > TendrilSink < tendril :: fmt :: UTF8 > for Parser < Sink > { fn process (& mut self , t : StrTendril) { self . input_buffer . push_back (t) ; while let TokenizerResult :: Script (_) = self . tokenizer . feed (& self . input_buffer) { } } fn error (& mut self , desc : Cow < 'static , str >) { self . tokenizer . sink . sink . parse_error (desc) } type Output = Sink :: Output ; fn finish (self) -> Self :: Output { while let TokenizerResult :: Script (_) = self . tokenizer . feed (& self . input_buffer) { } assert ! (self . input_buffer . is_empty ()) ; self . tokenizer . end () ; self . tokenizer . sink . sink . finish () } }
};
}
