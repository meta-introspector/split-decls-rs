// Generated macro for impl_14 (impl)
macro_rules! Depcrate_automockimpl_14 {
() => {
// Module: crate::automock
// Provides: {"impl_14"}
// Dependencies: {}
impl Parse for Attrs { fn parse (input : ParseStream) -> parse :: Result < Self > { let mut target = None ; let mut attrs = HashMap :: new () ; while ! input . is_empty () { let attr : Attr = input . parse () ? ; match attr { Attr :: Type (trait_item_type) => { let ident = trait_item_type . ident . clone () ; if let Some ((_ , ty)) = trait_item_type . default { attrs . insert (ident , ty . clone ()) ; } else { compile_error (trait_item_type . span () , "automock type attributes must have a default value") ; } } Attr :: Target (t) => { target . replace (t) ; } } } Ok (Attrs { target , attrs }) } }
};
}
