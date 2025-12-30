// Generated macro for relocalize_format_args_indexes (function)
macro_rules! Depcrate_write_literalrelocalize_format_args_indexes {
() => {
// Module: crate::write::literal
// Provides: {"relocalize_format_args_indexes"}
// Dependencies: {}
# [doc = " Relocalizes the indexes of positional arguments in the format string"] fn relocalize_format_args_indexes (piece : & FormatArgsPiece , suggestion : & mut Vec < (Span , String) > , replaced_position : & [usize] ,) { if let FormatArgsPiece :: Placeholder (FormatPlaceholder { argument : FormatArgPosition { index : Ok (index) , kind : FormatArgPositionKind :: Number , span : Some (span) , } , format_options , .. }) = piece { if suggestion . iter () . any (| (s , _) | s . overlaps (* span)) { return ; } let decremented_index = | index : usize | -> usize { let decrement = replaced_position . iter () . filter (| & & i | i < index) . count () ; index - decrement } ; suggestion . push ((* span , decremented_index (* index) . to_string ())) ; if * format_options != FormatOptions :: default () { let mut process_format_count = | count : & Option < FormatCount > , formatter : & dyn Fn (usize) -> String | { if let Some (FormatCount :: Argument (FormatArgPosition { index : Ok (format_arg_index) , kind : FormatArgPositionKind :: Number , span : Some (format_arg_span) , })) = count { suggestion . push ((* format_arg_span , formatter (decremented_index (* format_arg_index)))) ; } } ; process_format_count (& format_options . width , & | index : usize | format ! ("{index}$")) ; process_format_count (& format_options . precision , & | index : usize | format ! (".{index}$")) ; } } }
};
}
