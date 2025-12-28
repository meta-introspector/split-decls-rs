macro_rules! zlibVersion {
    () => {
        # [doc = " The version of the zlib library."] # [doc = ""] # [doc = " Its value is a pointer to a NULL-terminated sequence of bytes."] # [doc = ""] # [doc = " The version string for this release is `"] # [doc = libz_rs_sys_version ! ()] # [doc = " `:"] # [doc = ""] # [doc = " - The first component is the version of stock zlib that this release is compatible with"] # [doc = " - The final component is the zlib-rs version used to build this release."] # [cfg_attr (feature = "export-symbols" , export_name = prefix ! (zlibVersion))] pub const extern "C" fn zlibVersion () -> * const c_char { LIBZ_RS_SYS_VERSION . as_ptr () . cast :: < c_char > () }
    };
}

zlibVersion!()