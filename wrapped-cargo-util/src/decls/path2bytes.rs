macro_rules! path2bytes {
    () => {
        # [doc = " Converts a path to UTF-8 bytes."] pub fn path2bytes (path : & Path) -> Result < & [u8] > { # [cfg (unix)] { use std :: os :: unix :: prelude :: * ; Ok (path . as_os_str () . as_bytes ()) } # [cfg (windows)] { match path . as_os_str () . to_str () { Some (s) => Ok (s . as_bytes ()) , None => Err (anyhow :: format_err ! ("invalid non-unicode path: {}" , path . display ())) , } } }
    };
}

path2bytes!();