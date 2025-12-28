macro_rules! LineIndexDatabase {
    () => {
        # [query_group :: query_group] pub trait LineIndexDatabase : base_db :: RootQueryDb { # [salsa :: invoke_interned (line_index)] fn line_index (& self , file_id : FileId) -> Arc < LineIndex > ; }
    };
}

LineIndexDatabase!()