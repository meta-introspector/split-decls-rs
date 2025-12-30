// Generated macro for join_lines (function)
macro_rules! Depcrate_join_linesjoin_lines {
() => {
// Module: crate::join_lines
// Provides: {"join_lines"}
// Dependencies: {}
pub (crate) fn join_lines (config : & JoinLinesConfig , file : & SourceFile , range : TextRange ,) -> TextEdit { let range = if range . is_empty () { let syntax = file . syntax () ; let text = syntax . text () . slice (range . start () ..) ; let pos = match text . find_char ('\n') { None => return TextEdit :: builder () . finish () , Some (pos) => pos , } ; TextRange :: at (range . start () + pos , TextSize :: of ('\n')) } else { range } ; let mut edit = TextEdit :: builder () ; match file . syntax () . covering_element (range) { NodeOrToken :: Node (node) => { for token in node . descendants_with_tokens () . filter_map (| it | it . into_token ()) { remove_newlines (config , & mut edit , & token , range) } } NodeOrToken :: Token (token) => remove_newlines (config , & mut edit , & token , range) , } ; edit . finish () }
};
}
