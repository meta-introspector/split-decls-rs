// Generated macro for serialize (function)
macro_rules! Depcrate_serializeserialize {
() => {
// Module: crate::serialize
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < Wr : Write , T : Serializable > (writer : & mut Wr , node : & T , opts : SerializeOpts) -> io :: Result < () > { let mut ser = Serializer :: new (writer , opts) ; node . serialize (& mut ser , opts . traversal_scope) }
};
}
