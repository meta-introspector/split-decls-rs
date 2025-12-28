macro_rules! deps {
    () => {
        Stat!();
    };
}

macro_rules! Directory {
    () => {
        deps!();
        # [doc = " A directory with information about its untracked files, and its sub-directories"] # [derive (Clone)] pub struct Directory { # [doc = " The directories name, or an empty string if this is the root directory."] pub name : BString , # [doc = " Untracked files and directory names"] pub untracked_entries : Vec < BString > , # [doc = " indices for sub-directories similar to this one."] pub sub_directories : Vec < usize > , # [doc = " The directories stat data, if available or valid // TODO: or is it the exclude file?"] pub stat : Option < entry :: Stat > , # [doc = " The oid of a .gitignore file, if it exists"] pub exclude_file_oid : Option < ObjectId > , # [doc = " TODO: figure out what this really does"] pub check_only : bool , }
    };
}

Directory!();