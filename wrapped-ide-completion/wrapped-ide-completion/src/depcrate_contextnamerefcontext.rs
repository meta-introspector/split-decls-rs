// Generated macro for NameRefContext (struct)
macro_rules! Depcrate_contextNameRefContext {
() => {
// Module: crate::context
// Provides: {"NameRefContext"}
// Dependencies: {}
# [doc = " The state of the NameRef we are completing."] # [derive (Debug)] pub (crate) struct NameRefContext < 'db > { # [doc = " NameRef syntax in the original file"] pub (crate) nameref : Option < ast :: NameRef > , pub (crate) kind : NameRefKind < 'db > , }
};
}
