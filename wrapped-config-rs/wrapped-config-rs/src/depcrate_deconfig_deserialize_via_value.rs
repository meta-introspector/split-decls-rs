// Generated macro for config_deserialize_via_value (macro)
macro_rules! Depcrate_deconfig_deserialize_via_value {
() => {
// Module: crate::de
// Provides: {"config_deserialize_via_value"}
// Dependencies: {}
# [doc = " Define `$method`s, `deserialize_foo`, by forwarding to `Value`"] # [doc = ""] # [doc = " `($arg: $argtype, ...)`, if supplied, are the formal arguments"] macro_rules ! config_deserialize_via_value { { $ ($ method : ident $ (($ ($ arg : ident : $ argtype : ty) ,*)) ? ;) * } => { $ (# [inline] fn $ method < V : de :: Visitor <'de >> (self , $ ($ ($ arg : $ argtype ,) *) ? visitor : V ,) -> Result < V :: Value > { self . cache .$ method ($ ($ ($ arg ,) *) ? visitor) }) * } }
};
}
