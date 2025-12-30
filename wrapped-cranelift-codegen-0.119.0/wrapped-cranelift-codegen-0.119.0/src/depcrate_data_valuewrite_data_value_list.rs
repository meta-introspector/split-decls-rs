// Generated macro for write_data_value_list (function)
macro_rules! Depcrate_data_valuewrite_data_value_list {
() => {
// Module: crate::data_value
// Provides: {"write_data_value_list"}
// Dependencies: {}
# [doc = " Helper function for displaying `Vec<DataValue>`."] pub fn write_data_value_list (f : & mut Formatter < '_ > , list : & [DataValue]) -> fmt :: Result { match list . len () { 0 => Ok (()) , 1 => write ! (f , "{}" , list [0]) , _ => { write ! (f , "{}" , list [0]) ? ; for dv in list . iter () . skip (1) { write ! (f , ", {dv}") ? ; } Ok (()) } } }
};
}
