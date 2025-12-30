// Generated macro for t (macro)
macro_rules! Depcrate_testst {
() => {
// Module: crate::tests
// Provides: {"t"}
// Dependencies: {}
# [doc = " Short-hand constructor for SearchTest. We use it a lot below."] macro_rules ! t { ($ name : ident , $ patterns : expr , $ haystack : expr , $ matches : expr) => { SearchTest { name : stringify ! ($ name) , patterns : $ patterns , haystack : $ haystack , matches : $ matches , } } ; }
};
}
