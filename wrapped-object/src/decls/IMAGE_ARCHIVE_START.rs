macro_rules! IMAGE_ARCHIVE_START {
    () => {
        pub const IMAGE_ARCHIVE_START : & [u8 ; 8] = b"!<arch>\n" ;
    };
}

IMAGE_ARCHIVE_START!();