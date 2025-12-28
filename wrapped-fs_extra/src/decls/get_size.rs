macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! get_size {
    () => {
        deps!();
        # [doc = " Returns the size of the file or directory in bytes.(!important: folders size not count)"] # [doc = ""] # [doc = " If used on a directory, this function will recursively iterate over every file and every"] # [doc = " directory inside the directory. This can be very time consuming if used on large directories."] # [doc = ""] # [doc = " Does not follow symlinks."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `path` directory does not exist."] # [doc = " * Invalid `path`."] # [doc = " * The current process does not have the permission to access `path`."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::get_size;"] # [doc = ""] # [doc = " let folder_size = get_size(\"dir\")?;"] # [doc = " println!(\"{}\", folder_size); // print directory size in bytes"] # [doc = " ```"] pub fn get_size < P > (path : P) -> Result < u64 > where P : AsRef < Path > , { let path_metadata = path . as_ref () . symlink_metadata () ? ; let mut size_in_bytes = 0 ; if path_metadata . is_dir () { for entry in read_dir (& path) ? { let entry = entry ? ; let entry_metadata = entry . metadata () ? ; if entry_metadata . is_dir () { size_in_bytes += get_size (entry . path ()) ? ; } else { size_in_bytes += entry_metadata . len () ; } } } else { size_in_bytes = path_metadata . len () ; } Ok (size_in_bytes) }
    };
}

get_size!();