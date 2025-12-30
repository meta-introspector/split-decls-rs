// Generated macro for impl_54 (impl)
macro_rules! Depcrate_deimpl_54 {
() => {
// Module: crate::de
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'de , R , T > Iterator for StreamDeserializer < 'de , R , T > where R : Read < 'de > , T : de :: Deserialize < 'de > , { type Item = Result < T > ; fn next (& mut self) -> Option < Result < T > > { match self . de . peek () { Ok (Some (_)) => Some (T :: deserialize (& mut self . de)) , Ok (None) => None , Err (e) => Some (Err (e)) , } } }
};
}
