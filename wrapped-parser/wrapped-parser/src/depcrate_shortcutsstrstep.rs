// Generated macro for StrStep (enum)
macro_rules! Depcrate_shortcutsStrStep {
() => {
// Module: crate::shortcuts
// Provides: {"StrStep"}
// Dependencies: {}
# [derive (Debug)] pub enum StrStep < 'a > { Token { kind : SyntaxKind , text : & 'a str } , Enter { kind : SyntaxKind } , Exit , Error { msg : & 'a str , pos : usize } , }
};
}
