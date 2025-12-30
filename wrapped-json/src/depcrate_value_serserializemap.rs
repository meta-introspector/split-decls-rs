// Generated macro for SerializeMap (enum)
macro_rules! Depcrate_value_serSerializeMap {
() => {
// Module: crate::value::ser
// Provides: {"SerializeMap"}
// Dependencies: {}
pub enum SerializeMap { Map { map : Map < String , Value > , next_key : Option < String > , } , # [cfg (feature = "arbitrary_precision")] Number { out_value : Option < Value > } , # [cfg (feature = "raw_value")] RawValue { out_value : Option < Value > } , }
};
}
