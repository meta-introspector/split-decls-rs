macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! create_all {
    () => {
        deps!();
        # [doc = " Recursively create a directory and all of its parent components if they are missing."] # [doc = ""] # [doc = " This function takes to arguments:"] # [doc = ""] # [doc = " * `path` - Path to new directory."] # [doc = ""] # [doc = " * `erase` - If set true and folder exist, then folder will be erased."] # [doc = ""] # [doc = "#Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations,"] # [doc = " but is not limited to just these cases:"] # [doc = ""] # [doc = " * User lacks permissions to create directory at `path`."] # [doc = ""] # [doc = " * `path` already exists if `erase` set false."] # [doc = ""] # [doc = " #Examples"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::dir::create_all;"] # [doc = ""] # [doc = " create_all(\"/some/dir\", false); // create directory some and dir"] pub fn create_all < P > (path : P , erase : bool) -> Result < () > where P : AsRef < Path > , { if erase && path . as_ref () . exists () { remove (& path) ? ; } Ok (create_dir_all (& path) ?) }
    };
}

create_all!()