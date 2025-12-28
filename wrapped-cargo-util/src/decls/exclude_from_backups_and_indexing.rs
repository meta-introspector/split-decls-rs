macro_rules! exclude_from_backups_and_indexing {
    () => {
        # [doc = " Mark an existing directory as excluded from backups and indexing."] # [doc = ""] # [doc = " Errors in marking it are ignored."] pub fn exclude_from_backups_and_indexing (p : impl AsRef < Path >) { let path = p . as_ref () ; exclude_from_backups (path) ; exclude_from_content_indexing (path) ; }
    };
}

exclude_from_backups_and_indexing!()