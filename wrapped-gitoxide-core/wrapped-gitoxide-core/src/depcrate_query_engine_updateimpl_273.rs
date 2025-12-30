// Generated macro for impl_273 (impl)
macro_rules! Depcrate_query_engine_updateimpl_273 {
() => {
// Module: crate::query::engine::update
// Provides: {"impl_273"}
// Dependencies: {}
impl < 'a > Updates < 'a > { fn new (trans : & 'a Transaction < '_ >) -> rusqlite :: Result < Self > { let new_commit = trans . prepare (r#"INSERT INTO
               commits(hash)
               VALUES(?)"# ,) ? ; let insert_commit_file = trans . prepare (r#"
               INSERT INTO
               commit_file(hash, file_id, has_diff, lines_added, lines_removed, lines_before, lines_after, mode)
               VALUES(?, (SELECT files.file_id FROM files WHERE files.file_path = ?), ?, ?, ?, ?, ?, ?)
            "# ,) ? ; let insert_commit_file_with_source = trans . prepare (r#"
               INSERT INTO
               commit_file(hash, file_id, has_diff, lines_added, lines_removed, lines_before, lines_after, mode, source_file_id)
               VALUES(?, (SELECT files.file_id FROM files WHERE files.file_path = ?), ?, ?, ?, ?, ?, ?, (SELECT files.file_id FROM files WHERE files.file_path = ?))
            "# ,) ? ; let insert_file_path = trans . prepare (r#"
               INSERT OR IGNORE INTO
               files(file_path)
               VALUES(?)
            "# ,) ? ; Ok (Updates { new_commit , insert_commit_file , insert_commit_file_with_source , insert_file_path , }) } }
};
}
