macro_rules! deps {
    () => {
        Source!();
        State!();
        IgnoreMatchGroup!();
    };
}

macro_rules! Ignore {
    () => {
        deps!();
        # [doc = " State related to the exclusion of files, supporting static overrides and globals, along with a stack of dynamically read"] # [doc = " ignore files from disk or from the index each time the directory changes."] # [derive (Default , Clone)] # [allow (unused)] pub struct Ignore { # [doc = " Ignore patterns passed as overrides to everything else, typically passed on the command-line and the first patterns to"] # [doc = " be consulted."] overrides : IgnoreMatchGroup , # [doc = " Ignore patterns that match the currently set director (in the stack), which is pushed and popped as needed."] stack : IgnoreMatchGroup , # [doc = " Ignore patterns which aren't tied to the repository root, hence are global. They are consulted last."] globals : IgnoreMatchGroup , # [doc = " A matching stack of pattern indices which is empty if we have just been initialized to indicate that the"] # [doc = " currently set directory had a pattern matched. Note that this one could be negated."] # [doc = " (index into match groups, index into list of pattern lists, index into pattern list)"] matched_directory_patterns_stack : Vec < Option < (usize , usize , usize) > > , # [doc = "  The name of the file to look for in directories."] pub (crate) exclude_file_name_for_directories : BString , # [doc = " Where to read ignore files from"] source : ignore :: Source , # [doc = " Control how to parse ignore files."] parse : gix_ignore :: search :: Ignore , }
    };
}

Ignore!()