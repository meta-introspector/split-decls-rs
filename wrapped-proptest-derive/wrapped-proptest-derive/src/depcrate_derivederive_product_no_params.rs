// Generated macro for derive_product_no_params (function)
macro_rules! Depcrate_derivederive_product_no_params {
() => {
// Module: crate::derive
// Provides: {"derive_product_no_params"}
// Dependencies: {}
# [doc = " Deriving for a list of fields (product type) on"] # [doc = " which `params` or `no_params` was NOT set directly."] fn derive_product_no_params (ctx : Ctx , ut : & mut UseTracker , fields : Vec < Field > , item : & str ,) -> DeriveResult < PartsAcc < Ctor > > { let acc = PartsAcc :: new (fields . len ()) ; fields . into_iter () . try_fold (acc , | mut acc , field | { let attrs = attr :: parse_attributes (ctx , & field . attrs) ? ; error :: if_enum_attrs_present (ctx , & attrs , item) ; let span = field . span () ; let ty = field . ty ; let strat = pair_filter (attrs . filter , ty . clone () , match attrs . params { ParamsMode :: Passthrough => match attrs . strategy { StratMode :: Strategy (strat) => pair_existential (ty , strat) , StratMode :: Value (value) => pair_value (ty , value) , StratMode :: Regex (regex) => pair_regex (ty , regex) , StratMode :: Arbitrary => { ty . mark_uses (ut) ; let pref = acc . add_param (arbitrary_param (& ty)) ; pair_any_with (ty , pref , span) } } , ParamsMode :: Default => { product_handle_default_params (ut , ty , span , attrs . strategy) } ParamsMode :: Specified (params_ty) => { extract_nparam (& mut acc , params_ty , match attrs . strategy { StratMode :: Strategy (strat) => { pair_existential (ty , strat) } StratMode :: Value (value) => { pair_value_exist (ty , value) } StratMode :: Regex (regex) => { error :: cant_set_param_and_regex (ctx , item) ; pair_regex (ty , regex) } StratMode :: Arbitrary => { error :: cant_set_param_but_not_strat (ctx , & ty , item ,) ? } } ,) } } ,) ; Ok (acc . add_strat (strat)) }) }
};
}
