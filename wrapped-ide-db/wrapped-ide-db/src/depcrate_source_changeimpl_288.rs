// Generated macro for impl_288 (impl)
macro_rules! Depcrate_source_changeimpl_288 {
() => {
// Module: crate::source_change
// Provides: {"impl_288"}
// Dependencies: {}
impl FromIterator < (FileId , TextEdit) > for SourceChange { fn from_iter < T : IntoIterator < Item = (FileId , TextEdit) > > (iter : T) -> Self { let mut this = SourceChange :: default () ; this . extend (iter) ; this } }
};
}
