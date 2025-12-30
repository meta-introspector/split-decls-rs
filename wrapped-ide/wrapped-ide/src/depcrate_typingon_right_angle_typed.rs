// Generated macro for on_right_angle_typed (function)
macro_rules! Depcrate_typingon_right_angle_typed {
() => {
// Module: crate::typing
// Provides: {"on_right_angle_typed"}
// Dependencies: {}
# [doc = " Adds a space after an arrow when `fn foo() { ... }` is turned into `fn foo() -> { ... }`"] fn on_right_angle_typed (file : & SourceFile , offset : TextSize) -> Option < TextEdit > { let file_text = file . syntax () . text () ; let after_arrow = offset + TextSize :: of ('>') ; if file_text . char_at (after_arrow) != Some ('{') { return None ; } find_node_at_offset :: < ast :: RetType > (file . syntax () , offset) ? ; Some (TextEdit :: insert (after_arrow , " " . to_owned ())) }
};
}
