macro_rules! deps {
    () => {
        PathWrapper!();
    };
}

macro_rules! impl_11 {
    () => {
        deps!();
        impl PathWrapper { fn from_dir_entry (path : PathBuf , e : DirEntry) -> Self { let is_directory = e . file_type () . ok () . and_then (| file_type | { if file_type . is_symlink () { None } else { Some (file_type . is_dir ()) } }) . or_else (| | fs :: metadata (& path) . map (| m | m . is_dir ()) . ok ()) . unwrap_or (false) ; Self { path , is_directory } } fn from_path (path : PathBuf) -> Self { let is_directory = fs :: metadata (& path) . map (| m | m . is_dir ()) . unwrap_or (false) ; Self { path , is_directory } } fn into_path (self) -> PathBuf { self . path } }
    };
}

impl_11!();