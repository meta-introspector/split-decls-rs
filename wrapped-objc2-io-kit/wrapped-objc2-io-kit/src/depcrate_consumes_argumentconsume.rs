// Generated macro for consume (function)
macro_rules! Depcrate_consumes_argumentconsume {
() => {
// Module: crate::consumes_argument
// Provides: {"consume"}
// Dependencies: {}
fn consume (matching : Option < CFRetained < CFDictionary > >) -> * mut CFDictionary { if let Some (matching) = matching { CFRetained :: into_raw (matching) . as_ptr () } else { ptr :: null_mut () } }
};
}
