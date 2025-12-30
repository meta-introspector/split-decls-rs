// Generated macro for impl_255 (impl)
macro_rules! Depcrate_driverimpl_255 {
() => {
// Module: crate::driver
// Provides: {"impl_255"}
// Dependencies: {}
impl < Sink : TreeSink > TendrilSink < tendril :: fmt :: UTF8 > for Parser < Sink > { fn process (& mut self , t : StrTendril) { self . tokenizer . feed (t) } fn error (& mut self , desc : Cow < 'static , str >) { self . tokenizer . sink_mut () . sink_mut () . parse_error (desc) } type Output = Sink :: Output ; fn finish (mut self) -> Self :: Output { self . tokenizer . end () ; self . tokenizer . unwrap () . unwrap () . finish () } }
};
}
