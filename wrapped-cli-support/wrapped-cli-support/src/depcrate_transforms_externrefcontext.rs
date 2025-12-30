// Generated macro for Context (struct)
macro_rules! Depcrate_transforms_externrefContext {
() => {
// Module: crate::transforms::externref
// Provides: {"Context"}
// Dependencies: {}
# [doc = " State of the externref pass, used to collect information while bindings are"] # [doc = " generated and used eventually to actually execute the entire pass."] pub struct Context { imports : HashMap < ImportId , Function > , exports : HashMap < ExportId , Function > , table : TableId , bulk_memory : bool , }
};
}
