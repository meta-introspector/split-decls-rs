// Generated macro for for_all_argument_indexes (function)
macro_rules! Depcrate_formatfor_all_argument_indexes {
() => {
// Module: crate::format
// Provides: {"for_all_argument_indexes"}
// Dependencies: {}
fn for_all_argument_indexes (template : & mut [FormatArgsPiece] , mut f : impl FnMut (& mut usize)) { for piece in template { let FormatArgsPiece :: Placeholder (placeholder) = piece else { continue } ; if let Ok (index) = & mut placeholder . argument . index { f (index) ; } if let Some (FormatCount :: Argument (FormatArgPosition { index : Ok (index) , .. })) = & mut placeholder . format_options . width { f (index) ; } if let Some (FormatCount :: Argument (FormatArgPosition { index : Ok (index) , .. })) = & mut placeholder . format_options . precision { f (index) ; } } }
};
}
