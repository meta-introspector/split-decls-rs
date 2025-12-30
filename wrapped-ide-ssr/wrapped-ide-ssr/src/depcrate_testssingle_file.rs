// Generated macro for single_file (function)
macro_rules! Depcrate_testssingle_file {
() => {
// Module: crate::tests
// Provides: {"single_file"}
// Dependencies: {}
# [doc = " `code` may optionally contain a cursor marker `$0`. If it doesn't, then the position will be"] # [doc = " the start of the file. If there's a second cursor marker, then we'll return a single range."] pub (crate) fn single_file (code : & str) -> (ide_db :: RootDatabase , FilePosition , Vec < FileRange >) { use test_fixture :: { WORKSPACE , WithFixture } ; let (mut db , file_id , range_or_offset) = if code . contains (test_utils :: CURSOR_MARKER) { ide_db :: RootDatabase :: with_range_or_offset (code) } else { let (db , file_id) = ide_db :: RootDatabase :: with_single_file (code) ; (db , file_id , RangeOrOffset :: Offset (0 . into ())) } ; let selections ; let position ; match range_or_offset { RangeOrOffset :: Range (range) => { position = FilePosition { file_id , offset : range . start () } ; selections = vec ! [FileRange { file_id , range }] ; } RangeOrOffset :: Offset (offset) => { position = FilePosition { file_id , offset } ; selections = vec ! [] ; } } let mut local_roots = FxHashSet :: default () ; local_roots . insert (WORKSPACE) ; LocalRoots :: get (& db) . set_roots (& mut db) . to (local_roots) ; (db , position , selections) }
};
}
