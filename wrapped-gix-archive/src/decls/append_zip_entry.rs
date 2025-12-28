macro_rules! deps {
    () => {
        Error!();
    };
}

macro_rules! append_zip_entry {
    () => {
        deps!();
        # [cfg (feature = "zip")] fn append_zip_entry < W : std :: io :: Write + std :: io :: Seek > (ar : & mut zip :: write :: ZipWriter < W > , mut entry : gix_worktree_stream :: Entry < '_ > , buf : & mut Vec < u8 > , mtime : zip :: DateTime , compression_level : Option < i64 > , tree_prefix : Option < & bstr :: BString > ,) -> Result < () , Error > { let file_opts = zip :: write :: SimpleFileOptions :: default () . compression_method (zip :: CompressionMethod :: Deflated) . compression_level (compression_level) . large_file (entry . bytes_remaining () . is_none_or (| len | len > u32 :: MAX as usize)) . last_modified_time (mtime) . unix_permissions (if entry . mode . is_executable () { 0o755 } else { 0o644 }) ; let path = add_prefix (entry . relative_path () , tree_prefix) . into_owned () ; match entry . mode . kind () { gix_object :: tree :: EntryKind :: Blob | gix_object :: tree :: EntryKind :: BlobExecutable => { ar . start_file (path . to_string () , file_opts) . map_err (std :: io :: Error :: other) ? ; std :: io :: copy (& mut entry , ar) ? ; } gix_object :: tree :: EntryKind :: Tree | gix_object :: tree :: EntryKind :: Commit => { ar . add_directory (path . to_string () , file_opts) . map_err (std :: io :: Error :: other) ? ; } gix_object :: tree :: EntryKind :: Link => { use bstr :: ByteSlice ; std :: io :: copy (& mut entry , buf) ? ; ar . add_symlink (path . to_string () , buf . as_bstr () . to_string () , file_opts) . map_err (std :: io :: Error :: other) ? ; } } Ok (()) }
    };
}

append_zip_entry!()