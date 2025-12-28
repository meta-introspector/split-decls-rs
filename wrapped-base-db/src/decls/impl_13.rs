macro_rules! deps {
    () => {
        SourceRoot!();
    };
}

macro_rules! impl_13 {
    () => {
        deps!();
        impl SourceRoot { pub fn new_local (file_set : FileSet) -> SourceRoot { SourceRoot { is_library : false , file_set } } pub fn new_library (file_set : FileSet) -> SourceRoot { SourceRoot { is_library : true , file_set } } pub fn path_for_file (& self , file : & FileId) -> Option < & VfsPath > { self . file_set . path_for_file (file) } pub fn file_for_path (& self , path : & VfsPath) -> Option < & FileId > { self . file_set . file_for_path (path) } pub fn resolve_path (& self , path : AnchoredPath < '_ >) -> Option < FileId > { self . file_set . resolve_path (path) } pub fn iter (& self) -> impl Iterator < Item = FileId > + '_ { self . file_set . iter () } }
    };
}

impl_13!()