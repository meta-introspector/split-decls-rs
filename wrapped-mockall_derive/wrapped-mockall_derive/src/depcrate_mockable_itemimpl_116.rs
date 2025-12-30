// Generated macro for impl_116 (impl)
macro_rules! Depcrate_mockable_itemimpl_116 {
() => {
// Module: crate::mockable_item
// Provides: {"impl_116"}
// Dependencies: {}
impl From < ItemMod > for MockableModule { fn from (mod_ : ItemMod) -> MockableModule { let span = mod_ . span () ; let vis = mod_ . vis ; let mock_ident = format_ident ! ("mock_{}" , mod_ . ident) ; let orig_ident = Some (mod_ . ident) ; let content = if let Some ((_ , content)) = mod_ . content { content . into_iter () . map (mockable_item) . collect () } else { compile_error (span , "automock can only mock inline modules, not modules from another file") ; Vec :: new () } ; MockableModule { attrs : TokenStream :: new () , vis , mock_ident , orig_ident , content } } }
};
}
