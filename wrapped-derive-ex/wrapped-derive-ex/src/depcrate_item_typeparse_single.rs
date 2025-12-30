// Generated macro for parse_single (function)
macro_rules! Depcrate_item_typeparse_single {
() => {
// Module: crate::item_type
// Provides: {"parse_single"}
// Dependencies: {}
fn parse_single < T : Parse + Default > (attrs : & [Attribute] , name : & str) -> Result < Option < T > > { let mut item = None ; for attr in attrs { if attr . path () . is_ident (name) { if item . is_some () { bail ! (attr . span () , "#[{}] was specified twice" , name) } match & attr . meta { Meta :: Path (_) => item = Some (Default :: default ()) , Meta :: List (m) => item = Some (m . parse_args () ?) , Meta :: NameValue (_) => bail ! (attr . meta . span () , "`name = value` style attribute is not supported for `#[{}]" , name) , } } } Ok (item) }
};
}
