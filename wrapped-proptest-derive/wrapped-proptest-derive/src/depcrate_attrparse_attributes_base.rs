// Generated macro for parse_attributes_base (function)
macro_rules! Depcrate_attrparse_attributes_base {
() => {
// Module: crate::attr
// Provides: {"parse_attributes_base"}
// Dependencies: {}
# [doc = " Parse the attributes specified on an item and parsed by syn"] # [doc = " into our logical model that we work with."] fn parse_attributes_base (ctx : Ctx , attrs : & [Attribute] ,) -> DeriveResult < ParsedAttributes > { let acc = parse_accumulate (ctx , attrs) ; Ok (ParsedAttributes { skip : acc . skip . is_some () , weight : acc . weight , filter : acc . filter , params : parse_params_mode (ctx , acc . no_params , acc . params) ? , strategy : parse_strat_mode (ctx , acc . strategy , acc . value , acc . regex) ? , no_bound : acc . no_bound . is_some () , }) }
};
}
