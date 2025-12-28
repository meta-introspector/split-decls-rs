macro_rules! filename {
    () => {
        fn filename (path : & BStr) -> & BStr { path . rfind_byte (b'/') . map_or (path , | idx | path [idx + 1 ..] . as_bstr ()) }
    };
}

filename!()