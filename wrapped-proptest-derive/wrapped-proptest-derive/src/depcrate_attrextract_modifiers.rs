// Generated macro for extract_modifiers (function)
macro_rules! Depcrate_attrextract_modifiers {
() => {
// Module: crate::attr
// Provides: {"extract_modifiers"}
// Dependencies: {}
# [doc = " Extract all individual attributes inside one `#[proptest(..)]`."] # [doc = " We do this to treat all pieces uniformly whether a single"] # [doc = " `#[proptest(..)]` was used or many. This simplifies the"] # [doc = " logic somewhat."] fn extract_modifiers (ctx : Ctx , attr : & Attribute) -> Vec < Meta > { if ! is_outer_attr (& attr) { error :: inner_attr (ctx) ; } match & attr . meta { Meta :: List (list) => { if syn :: parse2 :: < Lit > (list . tokens . clone ()) . is_ok () { error :: immediate_literals (ctx) ; } else { let parser = Punctuated :: < Meta , Token ! [,] > :: parse_separated_nonempty ; let metas = parser . parse2 (list . tokens . clone ()) . unwrap () ; return metas . into_iter () . collect () ; } } Meta :: Path (_) => error :: bare_proptest_attr (ctx) , Meta :: NameValue (_) => error :: literal_set_proptest (ctx) , } vec ! [] }
};
}
