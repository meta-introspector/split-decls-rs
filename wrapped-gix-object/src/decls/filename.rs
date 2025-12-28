macro_rules! filename {
    () => {
        fn filename (path : & BStr) -> & BStr { path . rfind_byte (b'/') . map_or (path , | pos | & path [pos + 1 ..]) }
    };
}

filename!();