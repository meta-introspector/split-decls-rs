macro_rules! bytes2path {
    () => {
        # [doc = " Converts UTF-8 bytes to a path."] pub fn bytes2path (bytes : & [u8]) -> Result < PathBuf > { # [cfg (unix)] { use std :: os :: unix :: prelude :: * ; Ok (PathBuf :: from (OsStr :: from_bytes (bytes))) } # [cfg (windows)] { use std :: str ; match str :: from_utf8 (bytes) { Ok (s) => Ok (PathBuf :: from (s)) , Err (..) => Err (anyhow :: format_err ! ("invalid non-unicode path")) , } } }
    };
}

bytes2path!()