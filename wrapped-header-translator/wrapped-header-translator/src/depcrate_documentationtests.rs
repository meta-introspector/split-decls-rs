// Generated macro for tests (module)
macro_rules! Depcrate_documentationtests {
() => {
// Module: crate::documentation
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [track_caller] fn check (children : & [CommentChild] , expected : & str) { let actual = Documentation { first : None , from_header : children . to_vec () , extras : vec ! [] , alias : None , } . fmt (None) . to_string () ; assert_eq ! (actual , expected , "{children:?} was not") ; } # [test] fn format_simple () { let children = [CommentChild :: Paragraph (vec ! [CommentChild :: Text (" xyz." . into () ,)])] ; check (& children , "/// xyz.\n") ; let children = [CommentChild :: Paragraph (vec ! [CommentChild :: Text (" abc." . into ()) , CommentChild :: Text (" def." . into ()) ,])] ; check (& children , "/// abc.\n/// def.\n") ; } # [test] fn format_complex () { let children = [CommentChild :: Paragraph (vec ! [CommentChild :: Text ("\t" . into ())]) , CommentChild :: VerbatimLineCommand (" initWithLayoutTag:" . into ()) , CommentChild :: Paragraph (vec ! [CommentChild :: Text ("\t" . into ())]) , CommentChild :: BlockCommand (BlockCommand { command : "abstract" . into () , arguments : vec ! [] , children : vec ! [CommentChild :: Text (" Initialize from a layout tag." . into ()) , CommentChild :: Text ("\t" . into ()) ,] , }) , CommentChild :: ParamCommand (ParamCommand { index : Some (0) , parameter : "layoutTag" . into () , direction : None , children : vec ! [CommentChild :: Text ("\t\tThe tag." . into ()) , CommentChild :: Text ("\t" . into ()) ,] , }) , CommentChild :: BlockCommand (BlockCommand { command : "discussion" . into () , arguments : vec ! [] , children : vec ! [CommentChild :: Text ("\t\tReturns nil if the tag is either ABC or" . into ()) , CommentChild :: Text ("\t\tDEF." . into ()) ,] , }) ,] ; let expected = "/// Initialize from a layout tag.
///
/// Parameter `layoutTag`: The tag.
///
/// Returns nil if the tag is either ABC or
/// DEF.
" ; check (& children , expected) ; let children = [CommentChild :: Paragraph (vec ! [CommentChild :: Text (" " . into ())]) , CommentChild :: VerbatimLineCommand (" lengthInBeats" . into ()) , CommentChild :: Paragraph (vec ! [CommentChild :: Text ("\t" . into ())]) , CommentChild :: BlockCommand (BlockCommand { command : "abstract" . into () , arguments : vec ! [] , children : vec ! [CommentChild :: Text (" XYZ" . into ()) , CommentChild :: Text ("\t" . into ()) ,] , }) , CommentChild :: BlockCommand (BlockCommand { command : "discussion" . into () , arguments : vec ! [] , children : vec ! [CommentChild :: Text ("\t\tA" . into ()) , CommentChild :: Text ("\t\tB" . into ()) , CommentChild :: Text ("\t\tC." . into ()) ,] , }) , CommentChild :: Paragraph (vec ! [CommentChild :: Text ("\t\tD." . into ())]) ,] ; let expected = "/// XYZ
///
/// A
/// B
/// C.
///
/// D.
" ; check (& children , expected) ; let children = [CommentChild :: Paragraph (vec ! [CommentChild :: Text (" " . into ())]) , CommentChild :: VerbatimLineCommand (" serializeToURL:error:" . into ()) , CommentChild :: Paragraph (vec ! [CommentChild :: Text (" " . into ())]) , CommentChild :: BlockCommand (BlockCommand { command : "abstract" . into () , arguments : vec ! [] , children : vec ! [CommentChild :: Text (" XYZ." . into ()) , CommentChild :: Text (" " . into ()) ,] , }) , CommentChild :: BlockCommand (BlockCommand { command : "discussion" . into () , arguments : vec ! [] , children : vec ! [CommentChild :: Text (" A." . into ()) , CommentChild :: Text (" B." . into ()) , CommentChild :: Text (" C." . into ()) , CommentChild :: Text (" " . into ()) ,] , }) , CommentChild :: ParamCommand (ParamCommand { index : Some (0) , parameter : "url" . into () , direction : None , children : vec ! [CommentChild :: Text (" E." . into ()) , CommentChild :: Text (" " . into ()) ,] , }) , CommentChild :: ParamCommand (ParamCommand { index : Some (1) , parameter : "error" . into () , direction : None , children : vec ! [CommentChild :: Text (" F." . into ()) , CommentChild :: Text (" " . into ()) ,] , }) , CommentChild :: BlockCommand (BlockCommand { command : "return" . into () , arguments : vec ! [] , children : vec ! [CommentChild :: Text (" G." . into ())] , }) ,] ; let expected = "/// XYZ.
///
/// A.
/// B.
/// C.
///
/// Parameter `url`: E.
///
/// Parameter `error`: F.
///
/// Returns: G.
" ; check (& children , expected) ; } # [test] fn multiline_with_internal_tabs () { let children = [CommentChild :: Paragraph (vec ! [CommentChild :: Text ("\tA" . into ()) , CommentChild :: Text ("\tB." . into ()) ,]) , CommentChild :: Paragraph (vec ! [CommentChild :: Text ("\tC." . into ())]) ,] ; let expected = "/// A\n/// B.\n///\n/// C.\n" ; check (& children , expected) ; } }
};
}
