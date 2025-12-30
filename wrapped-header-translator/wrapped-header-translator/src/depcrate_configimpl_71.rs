// Generated macro for impl_71 (impl)
macro_rules! Depcrate_configimpl_71 {
() => {
// Module: crate::config
// Provides: {"impl_71"}
// Dependencies: {}
impl < 'de > de :: Deserialize < 'de > for Counterpart { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : de :: Deserializer < 'de > , { struct CounterpartVisitor ; impl de :: Visitor < '_ > for CounterpartVisitor { type Value = Counterpart ; fn expecting (& self , formatter : & mut fmt :: Formatter) -> fmt :: Result { formatter . write_str ("item identifier") } fn visit_str < E > (self , value : & str) -> Result < Self :: Value , E > where E : de :: Error , { if let Some (value) = value . strip_prefix ("ImmutableSuperclass(") { let value = value . strip_suffix (')') . ok_or_else (| | de :: Error :: custom ("end parenthesis")) ? ; let item = ItemIdentifier :: from_str (value) . map_err (de :: Error :: custom) ? ; return Ok (Counterpart :: ImmutableSuperclass (item)) ; } if let Some (value) = value . strip_prefix ("MutableSubclass(") { let value = value . strip_suffix (')') . ok_or_else (| | de :: Error :: custom ("end parenthesis")) ? ; let item = ItemIdentifier :: from_str (value) . map_err (de :: Error :: custom) ? ; return Ok (Counterpart :: MutableSubclass (item)) ; } Err (de :: Error :: custom (format ! ("unknown variant {value:?}"))) } } deserializer . deserialize_str (CounterpartVisitor) } }
};
}
