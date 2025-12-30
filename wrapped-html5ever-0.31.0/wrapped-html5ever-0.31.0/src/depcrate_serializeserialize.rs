// Generated macro for serialize (function)
macro_rules! Depcrate_serializeserialize {
() => {
// Module: crate::serialize
// Provides: {"serialize"}
// Dependencies: {}
pub fn serialize < Wr , T > (writer : Wr , node : & T , opts : SerializeOpts) -> io :: Result < () > where Wr : Write , T : Serialize , { let mut ser = HtmlSerializer :: new (writer , opts . clone ()) ; node . serialize (& mut ser , opts . traversal_scope) }
};
}
