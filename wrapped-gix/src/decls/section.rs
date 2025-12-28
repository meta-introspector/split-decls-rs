macro_rules! deps {
    () => {
        Kind!();
        Repository!();
    };
}

macro_rules! section {
    () => {
        deps!();
        # [doc = ""] pub mod section { # [doc = " A filter that returns `true` for `meta` if the meta-data attached to a configuration section can be trusted."] # [doc = " This is either the case if its file is fully trusted, or if it's a section from a system-wide file."] pub fn is_trusted (meta : & gix_config :: file :: Metadata) -> bool { meta . trust == gix_sec :: Trust :: Full || meta . source . kind () != gix_config :: source :: Kind :: Repository } }
    };
}

section!();