// Generated macro for impl_248 (impl)
macro_rules! Depcrate_options_filterimpl_248 {
() => {
// Module: crate::options::filter
// Provides: {"impl_248"}
// Dependencies: {}
impl SortField { # [doc = " Determines which sort field to use based on the `--sort` argument."] # [doc = " This argument’s value can be one of several flags, listed above."] # [doc = " Returns the default sort field if none is given, or `Err` if the"] # [doc = " value doesn’t correspond to a sort field we know about."] fn deduce (matches : & MatchedFlags < '_ >) -> Result < Self , OptionsError > { let Some (word) = matches . get (& flags :: SORT) ? else { return Ok (Self :: default ()) ; } ; let Some (word) = word . to_str () else { return Err (OptionsError :: BadArgument (& flags :: SORT , word . into ())) ; } ; let field = match word { "name" | "filename" => Self :: Name (SortCase :: AaBbCc) , "Name" | "Filename" => Self :: Name (SortCase :: ABCabc) , ".name" | ".filename" => Self :: NameMixHidden (SortCase :: AaBbCc) , ".Name" | ".Filename" => Self :: NameMixHidden (SortCase :: ABCabc) , "size" | "filesize" => Self :: Size , "ext" | "extension" => Self :: Extension (SortCase :: AaBbCc) , "Ext" | "Extension" => Self :: Extension (SortCase :: ABCabc) , "date" | "time" | "mod" | "modified" | "new" | "newest" => Self :: ModifiedDate , "age" | "old" | "oldest" => Self :: ModifiedAge , "ch" | "changed" => Self :: ChangedDate , "acc" | "accessed" => Self :: AccessedDate , "cr" | "created" => Self :: CreatedDate , # [cfg (unix)] "inode" => Self :: FileInode , "type" => Self :: FileType , "none" => Self :: Unsorted , _ => { return Err (OptionsError :: BadArgument (& flags :: SORT , word . into ())) ; } } ; Ok (field) } }
};
}
