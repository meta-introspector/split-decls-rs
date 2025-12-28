macro_rules! let_readers_execute {
    () => {
        # [doc = " Given the st_mode of a regular file, compute the mode with executable bits safely added."] # [doc = ""] # [doc = " Currently this adds executable bits for whoever has read bits already. It doesn't use the umask."] # [doc = " Set-user-ID and set-group-ID bits are unset for safety. The sticky bit is also unset."] # [doc = ""] # [doc = " This returns only mode bits, not file type. The return value can be used in chmod or fchmod."] # [cfg (any (unix , test))] fn let_readers_execute (mut mode : u32) -> u32 { assert_eq ! (mode & 0o170000 , 0o100000 , "bug in caller if not from a regular file") ; mode &= 0o777 ; mode |= (mode & 0o444) >> 2 ; mode }
    };
}

let_readers_execute!();