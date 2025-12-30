// Generated macro for do_choice (macro)
macro_rules! Depcrate_parser_choicedo_choice {
() => {
// Module: crate::parser::choice
// Provides: {"do_choice"}
// Dependencies: {}
macro_rules ! do_choice { ($ input : ident $ before_position : ident $ before : ident $ partial_state : ident $ state : ident () $ ($ parser : ident $ error : ident) +) => { { let mut error = Tracked :: from (merge ! ($ ($ error) +)) ; $ (if $ error . offset != ErrorOffset (1) { error . offset = $ error . offset ; $ parser . add_error (& mut error) ; error . offset = ErrorOffset (0) ; }) + PeekErr (error) } } ; ($ input : ident $ before_position : ident $ before : ident $ partial_state : ident $ state : ident ($ head : ident $ ($ tail : ident) *) $ ($ all : ident) *) => { { let parser = $ head ; let mut state = $ head :: PartialState :: default () ; match parser . parse_mode (crate :: parser :: FirstMode , $ input , & mut state) { CommitOk (x) => CommitOk (x) , PeekOk (x) => PeekOk (x) , CommitErr (err) => { if $ input . position () != $ before_position { *$ state = self ::$ partial_state ::$ head (state) ; } CommitErr (err) } PeekErr ($ head) => { ctry ! ($ input . reset ($ before . clone ()) . committed ()) ; do_choice ! ($ input $ before_position $ before $ partial_state $ state ($ ($ tail) *) $ ($ all) * parser $ head) } } } } }
};
}
