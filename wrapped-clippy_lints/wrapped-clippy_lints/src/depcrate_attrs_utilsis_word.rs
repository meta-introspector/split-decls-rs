// Generated macro for is_word (function)
macro_rules! Depcrate_attrs_utilsis_word {
() => {
// Module: crate::attrs::utils
// Provides: {"is_word"}
// Dependencies: {}
pub (super) fn is_word (nmi : & MetaItemInner , expected : Symbol) -> bool { if let MetaItemInner :: MetaItem (mi) = & nmi { mi . is_word () && mi . has_name (expected) } else { false } }
};
}
