// Generated macro for impl_1894 (impl)
macro_rules! Depcrate_empty_line_afterimpl_1894 {
() => {
// Module: crate::empty_line_after
// Provides: {"impl_1894"}
// Dependencies: {}
impl EarlyLintPass for EmptyLineAfter { fn check_crate (& mut self , _ : & EarlyContext < '_ > , krate : & Crate) { self . items . push (ItemInfo { kind : "crate" , name : Some (kw :: Crate) , span : krate . spans . inner_span . with_hi (krate . spans . inner_span . lo ()) , mod_items : krate . items . iter () . filter (| i | ! matches ! (i . span . ctxt () . outer_expn_data () . kind , ExpnKind :: AstPass (_))) . map (| i | i . id) . next () , }) ; } fn check_item_post (& mut self , _ : & EarlyContext < '_ > , _ : & Item) { self . items . pop () ; } fn check_impl_item_post (& mut self , _ : & EarlyContext < '_ > , _ : & Item < AssocItemKind >) { self . items . pop () ; } fn check_trait_item_post (& mut self , _ : & EarlyContext < '_ > , _ : & Item < AssocItemKind >) { self . items . pop () ; } fn check_impl_item (& mut self , cx : & EarlyContext < '_ > , item : & Item < AssocItemKind >) { self . check_item_kind (cx , & item . kind . clone () . into () , item . kind . ident () , item . span , & item . attrs , item . id ,) ; } fn check_trait_item (& mut self , cx : & EarlyContext < '_ > , item : & Item < AssocItemKind >) { self . check_item_kind (cx , & item . kind . clone () . into () , item . kind . ident () , item . span , & item . attrs , item . id ,) ; } fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & Item) { self . check_item_kind (cx , & item . kind , item . kind . ident () , item . span , & item . attrs , item . id) ; } }
};
}
