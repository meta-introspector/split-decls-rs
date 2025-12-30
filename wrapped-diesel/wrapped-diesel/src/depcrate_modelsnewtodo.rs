// Generated macro for NewTodo (struct)
macro_rules! Depcrate_modelsNewTodo {
() => {
// Module: crate::models
// Provides: {"NewTodo"}
// Dependencies: {}
# [derive (Insertable)] # [diesel (table_name = crate :: schema :: todos)] pub struct NewTodo { pub title : CompactString , pub done : bool , }
};
}
