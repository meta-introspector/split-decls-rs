// Generated macro for hash_recursive (function)
macro_rules! Depcratehash_recursive {
() => {
// Module: crate
// Provides: {"hash_recursive"}
// Dependencies: {}
# [doc = " Hash the metadata and size of every file in a directory, recursively."] fn hash_recursive (path : & Path , hasher : & mut DefaultHasher) -> Result < () > { for entry in WalkDir :: new (path) . follow_links (true) . sort_by_file_name () . into_iter () { let entry = entry ? ; if entry . file_type () . is_dir () { continue ; } let meta = entry . metadata () ? ; meta . modified () ? . hash (hasher) ; meta . len () . hash (hasher) ; } Ok (()) }
};
}
