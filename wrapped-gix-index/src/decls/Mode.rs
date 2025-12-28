macro_rules! Mode {
    () => {
        # [doc = " Describes the configuration how a sparse index should be written, or if one should be written at all."] # [derive (Debug)] pub enum Mode { # [doc = " index with DIR entries for exclusion and included entries, directory-only include patterns in `.git/info/sparse-checkout` file."] IncludeDirectoriesStoreIncludedEntriesAndExcludedDirs , # [doc = " index with all file entries and skip worktree flags for exclusion, directory-only include patterns in `.git/info/sparse-checkout` file."] IncludeDirectoriesStoreAllEntriesSkipUnmatched , # [doc = " index with all file entries and skip-worktree flags for exclusion, `ignore` patterns to include entries in `.git/info/sparse-checkout` file."] IncludeByIgnorePatternStoreAllEntriesSkipUnmatched , # [doc = " index with all entries, non is excluded, `.git/info/sparse-checkout` file is not considered, a regular index."] Disabled , }
    };
}

Mode!();