macro_rules! deps {
    () => {
        CopyOptions!();
        Result!();
    };
}

macro_rules! move_file {
    () => {
        deps!();
        # [doc = " Moves a file from one place to another. This function will also copy the permission"] # [doc = " bits of the original file to the destination file."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `from` path is not a file."] # [doc = " * This `from` file does not exist."] # [doc = " * The current process does not have the permission to access `from` or write `to`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::file::move_file;"] # [doc = ""] # [doc = " let options = CopyOptions::new(); //Initialize default values for CopyOptions"] # [doc = " move_file(\"dir1/foo.txt\", \"dir2/foo.txt\", &options)?; // Move dir1/foo.txt to dir2/foo.txt"] # [doc = ""] # [doc = " ```"] pub fn move_file < P , Q > (from : P , to : Q , options : & CopyOptions) -> Result < u64 > where P : AsRef < Path > , Q : AsRef < Path > , { let mut is_remove = true ; if options . skip_exist && to . as_ref () . exists () && ! options . overwrite { is_remove = false ; } let result = copy (& from , to , options) ? ; if is_remove { remove (from) ? ; } Ok (result) }
    };
}

move_file!();