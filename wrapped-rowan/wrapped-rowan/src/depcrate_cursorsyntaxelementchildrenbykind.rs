// Generated macro for SyntaxElementChildrenByKind (struct)
macro_rules! Depcrate_cursorSyntaxElementChildrenByKind {
() => {
// Module: crate::cursor
// Provides: {"SyntaxElementChildrenByKind"}
// Dependencies: {}
# [derive (Clone , Debug)] pub struct SyntaxElementChildrenByKind < F : Fn (SyntaxKind) -> bool > { next : Option < SyntaxElement > , matcher : F , }
};
}
