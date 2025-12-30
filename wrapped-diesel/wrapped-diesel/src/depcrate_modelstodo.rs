// Generated macro for Todo (struct)
macro_rules! Depcrate_modelsTodo {
() => {
// Module: crate::models
// Provides: {"Todo"}
// Dependencies: {}
# [derive (Queryable , Selectable)] # [diesel (table_name = crate :: schema :: todos)] # [diesel (check_for_backend (diesel :: sqlite :: Sqlite))] pub struct Todo { pub id : i32 , pub title : CompactString , pub done : bool , }
};
}
