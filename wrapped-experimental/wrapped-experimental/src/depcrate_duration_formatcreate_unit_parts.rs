// Generated macro for create_unit_parts (macro)
macro_rules! Depcrate_duration_formatcreate_unit_parts {
() => {
// Module: crate::duration::format
// Provides: {"create_unit_parts"}
// Dependencies: {}
macro_rules ! create_unit_parts { ($ ($ part : ident , $ unit : expr) ,*) => { $ (pub const $ part : Part = Part { category : "unit" , value : $ unit . as_unit_formatter_name () , } ;) * } ; }
};
}
