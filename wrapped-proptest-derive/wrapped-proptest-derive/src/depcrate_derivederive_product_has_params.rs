// Generated macro for derive_product_has_params (function)
macro_rules! Depcrate_derivederive_product_has_params {
() => {
// Module: crate::derive
// Provides: {"derive_product_has_params"}
// Dependencies: {}
# [doc = " Deriving for a list of fields (product type) on"] # [doc = " which `params` or `no_params` was set directly."] fn derive_product_has_params (ctx : Ctx , ut : & mut UseTracker , item : & str , closure : MapClosure , fields : Vec < Field > ,) -> DeriveResult < StratPair > { let len = fields . len () ; fields . into_iter () . try_fold (StratAcc :: new (len) , | acc , field | { let attrs = attr :: parse_attributes (ctx , & field . attrs) ? ; error :: if_enum_attrs_present (ctx , & attrs , item) ; error :: if_specified_params (ctx , & attrs , item) ; let span = field . span () ; let ty = field . ty . clone () ; let pair = product_handle_default_params (ut , ty , span , attrs . strategy) ; let pair = pair_filter (attrs . filter , field . ty , pair) ; Ok (acc . add (pair)) }) . map (| acc | acc . finish (closure)) }
};
}
