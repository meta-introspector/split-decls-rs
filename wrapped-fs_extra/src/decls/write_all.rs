macro_rules! deps {
    () => {
        Result!();
        ErrorKind!();
    };
}

macro_rules! write_all {
    () => {
        deps!();
        # [doc = " Write `String` content into file."] # [doc = ""] # [doc = " # Errors"] # [doc = ""] # [doc = " This function will return an error in the following situations, but is not limited to just"] # [doc = " these cases:"] # [doc = ""] # [doc = " * This `path` is not a file."] # [doc = " * This `path` file does not exist."] # [doc = " * The current process does not have the permission to access `path`."] # [doc = ""] # [doc = " # Example"] # [doc = " ```rust,ignore"] # [doc = " extern crate fs_extra;"] # [doc = " use fs_extra::file::read_to_string;"] # [doc = ""] # [doc = " write_all(\"foo.txt\", \"contents\" )?; // Create file foo.txt and send content inside"] # [doc = ""] # [doc = " ```"] pub fn write_all < P > (path : P , content : & str) -> Result < () > where P : AsRef < Path > , { let path = path . as_ref () ; if path . exists () && ! path . is_file () { if let Some (msg) = path . to_str () { let msg = format ! ("Path \"{}\" is not a file!" , msg) ; err ! (& msg , ErrorKind :: InvalidFile) ; } err ! ("Path is not a file!" , ErrorKind :: InvalidFile) ; } let mut f = File :: create (path) ? ; Ok (f . write_all (content . as_bytes ()) ?) }
    };
}

write_all!()