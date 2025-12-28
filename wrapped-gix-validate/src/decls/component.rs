macro_rules! deps {
    () => {
        Mode!();
    };
}

macro_rules! component {
    () => {
        deps!();
        # [doc = " Assure the given `input` resembles a valid name for a tree or blob, and in that sense, a path component."] # [doc = " `mode` indicates the kind of `input` and it should be `Some` if `input` is the last component in the underlying"] # [doc = " path."] # [doc = ""] # [doc = " `input` must not make it possible to exit the repository, or to specify absolute paths."] pub fn component (input : & BStr , mode : Option < component :: Mode > , component :: Options { protect_windows , protect_hfs , protect_ntfs , } : component :: Options ,) -> Result < & BStr , component :: Error > { if input . is_empty () { return Err (component :: Error :: Empty) ; } if input == ".." || input == "." { return Err (component :: Error :: Relative) ; } if protect_windows { if input . find_byteset (br"/\") . is_some () { return Err (component :: Error :: PathSeparator) ; } if input . chars () . nth (1) == Some (':') { return Err (component :: Error :: WindowsPathPrefix) ; } } else if input . find_byte (b'/') . is_some () { return Err (component :: Error :: PathSeparator) ; } if protect_hfs { if is_dot_hfs (input , "git") { return Err (component :: Error :: DotGitDir) ; } if is_symlink (mode) && is_dot_hfs (input , "gitmodules") { return Err (component :: Error :: SymlinkedGitModules) ; } } if protect_ntfs { if is_dot_git_ntfs (input) { return Err (component :: Error :: DotGitDir) ; } if is_symlink (mode) && is_dot_ntfs (input , "gitmodules" , "gi7eba") { return Err (component :: Error :: SymlinkedGitModules) ; } if protect_windows { if let Some (err) = check_win_devices_and_illegal_characters (input) { return Err (err) ; } } } if ! (protect_hfs | protect_ntfs) { if input . eq_ignore_ascii_case (b".git") { return Err (component :: Error :: DotGitDir) ; } if is_symlink (mode) && input . eq_ignore_ascii_case (b".gitmodules") { return Err (component :: Error :: SymlinkedGitModules) ; } } Ok (input) }
    };
}

component!()