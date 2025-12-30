// Generated macro for assists (function)
macro_rules! Depcrateassists {
() => {
// Module: crate
// Provides: {"assists"}
// Dependencies: {}
# [doc = " Return all the assists applicable at the given position."] # [doc = ""] pub fn assists (db : & RootDatabase , config : & AssistConfig , resolve : AssistResolveStrategy , range : ide_db :: FileRange ,) -> Vec < Assist > { let sema = Semantics :: new (db) ; let file_id = sema . attach_first_edition (range . file_id) . unwrap_or_else (| | EditionedFileId :: new (db , range . file_id , Edition :: CURRENT)) ; let ctx = AssistContext :: new (sema , config , hir :: FileRange { file_id , range : range . range }) ; let mut acc = Assists :: new (& ctx , resolve) ; handlers :: all () . iter () . for_each (| handler | { handler (& mut acc , & ctx) ; }) ; acc . finish () }
};
}
