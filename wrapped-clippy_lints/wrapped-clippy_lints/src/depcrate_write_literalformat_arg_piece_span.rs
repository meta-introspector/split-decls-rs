// Generated macro for format_arg_piece_span (function)
macro_rules! Depcrate_write_literalformat_arg_piece_span {
() => {
// Module: crate::write::literal
// Provides: {"format_arg_piece_span"}
// Dependencies: {}
# [doc = " Extract Span and its index from the given `piece`"] fn format_arg_piece_span (piece : & FormatArgsPiece) -> Option < (Span , usize) > { match piece { FormatArgsPiece :: Placeholder (FormatPlaceholder { argument : FormatArgPosition { index : Ok (index) , .. } , span : Some (span) , .. }) => Some ((* span , * index)) , _ => None , } }
};
}
