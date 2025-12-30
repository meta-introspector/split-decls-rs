// Generated macro for move_item (function)
macro_rules! Depcrate_move_itemmove_item {
() => {
// Module: crate::move_item
// Provides: {"move_item"}
// Dependencies: {}
pub (crate) fn move_item (db : & RootDatabase , range : FileRange , direction : Direction ,) -> Option < TextEdit > { let sema = Semantics :: new (db) ; let file = sema . parse_guess_edition (range . file_id) ; let item = if range . range . is_empty () { SyntaxElement :: Token (pick_best_token (file . syntax () . token_at_offset (range . range . start ()) , | kind | match kind { SyntaxKind :: IDENT | SyntaxKind :: LIFETIME_IDENT => 2 , kind if kind . is_trivia () => 0 , _ => 1 , } ,) ?) } else { file . syntax () . covering_element (range . range) } ; find_ancestors (item , direction , range . range) }
};
}
