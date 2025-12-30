// Generated macro for sub_section (module)
macro_rules! Depcrate_parse_nom_testssub_section {
() => {
// Module: crate::parse::nom::tests
// Provides: {"sub_section"}
// Dependencies: {}
mod sub_section { use std :: borrow :: Cow ; use winnow :: prelude :: * ; use super :: sub_section ; # [test] fn zero_copy_simple () { let actual = sub_section . parse_peek (br#"name""#) . unwrap () . 1 ; assert_eq ! (actual . as_ref () , "name") ; assert ! (matches ! (actual , Cow :: Borrowed (_))) ; } # [test] fn escapes_need_allocation () { let actual = sub_section . parse_peek (br#"\x\t\n\0\\\"""#) . unwrap () . 1 ; assert_eq ! (actual . as_ref () , r#"xtn0\""#) ; assert ! (matches ! (actual , Cow :: Owned (_))) ; } }
};
}
