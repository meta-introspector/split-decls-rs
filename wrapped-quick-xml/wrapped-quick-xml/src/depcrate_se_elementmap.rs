// Generated macro for Map (struct)
macro_rules! Depcrate_se_elementMap {
() => {
// Module: crate::se::element
// Provides: {"Map"}
// Dependencies: {}
pub struct Map < 'w , 'k , W : Write > { ser : Struct < 'w , 'k , W > , # [doc = " Key, serialized by `QNameSerializer` if consumer uses `serialize_key` +"] # [doc = " `serialize_value` calls instead of `serialize_entry`"] key : Option < String > , }
};
}
