// Generated macro for parse_selection_set (function)
macro_rules! Depcrate_parse_executableparse_selection_set {
() => {
// Module: crate::parse::executable
// Provides: {"parse_selection_set"}
// Dependencies: {}
fn parse_selection_set (pair : Pair < Rule > , pc : & mut PositionCalculator , remaining_depth : usize ,) -> Result < Positioned < SelectionSet > > { debug_assert_eq ! (pair . as_rule () , Rule :: selection_set) ; let pos = pc . step (& pair) ; Ok (Positioned :: new (SelectionSet { items : pair . into_inner () . map (| pair | parse_selection (pair , pc , remaining_depth)) . collect :: < Result < _ > > () ? , } , pos ,)) }
};
}
