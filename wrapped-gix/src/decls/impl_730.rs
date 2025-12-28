macro_rules! deps {
    () => {
        Safe!();
        Kind!();
    };
}

macro_rules! impl_730 {
    () => {
        deps!();
        impl Safe { # [doc = " Implements the directory filter to trust only global and system files, for use with `safe.directory`."] pub fn directory_filter (meta : & gix_config :: file :: Metadata) -> bool { let kind = meta . source . kind () ; kind == gix_config :: source :: Kind :: System || kind == gix_config :: source :: Kind :: Global } }
    };
}

impl_730!();