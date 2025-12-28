macro_rules! deps {
    () => {
        SourceRootInput!();
        SourceRootId!();
        Files!();
        SourceRoot!();
        FileSourceRootInput!();
        SourceDatabase!();
        FileText!();
    };
}

macro_rules! impl_81 {
    () => {
        deps!();
        impl Files { pub fn file_text (& self , file_id : vfs :: FileId) -> FileText { match self . files . get (& file_id) { Some (text) => * text , None => { panic ! ("Unable to fetch file text for `vfs::FileId`: {file_id:?}; this is a bug") } } } pub fn set_file_text (& self , db : & mut dyn SourceDatabase , file_id : vfs :: FileId , text : & str) { match self . files . entry (file_id) { Entry :: Occupied (mut occupied) => { occupied . get_mut () . set_text (db) . to (Arc :: from (text)) ; } Entry :: Vacant (vacant) => { let text = FileText :: new (db , Arc :: from (text) , file_id) ; vacant . insert (text) ; } } ; } pub fn set_file_text_with_durability (& self , db : & mut dyn SourceDatabase , file_id : vfs :: FileId , text : & str , durability : Durability ,) { match self . files . entry (file_id) { Entry :: Occupied (mut occupied) => { occupied . get_mut () . set_text (db) . with_durability (durability) . to (Arc :: from (text)) ; } Entry :: Vacant (vacant) => { let text = FileText :: builder (Arc :: from (text) , file_id) . durability (durability) . new (db) ; vacant . insert (text) ; } } ; } # [doc = " Source root of the file."] pub fn source_root (& self , source_root_id : SourceRootId) -> SourceRootInput { let source_root = match self . source_roots . get (& source_root_id) { Some (source_root) => source_root , None => panic ! ("Unable to fetch `SourceRootInput` with `SourceRootId` ({source_root_id:?}); this is a bug") , } ; * source_root } pub fn set_source_root_with_durability (& self , db : & mut dyn SourceDatabase , source_root_id : SourceRootId , source_root : Arc < SourceRoot > , durability : Durability ,) { match self . source_roots . entry (source_root_id) { Entry :: Occupied (mut occupied) => { occupied . get_mut () . set_source_root (db) . with_durability (durability) . to (source_root) ; } Entry :: Vacant (vacant) => { let source_root = SourceRootInput :: builder (source_root) . durability (durability) . new (db) ; vacant . insert (source_root) ; } } ; } pub fn file_source_root (& self , id : vfs :: FileId) -> FileSourceRootInput { let file_source_root = match self . file_source_roots . get (& id) { Some (file_source_root) => file_source_root , None => panic ! ("Unable to get `FileSourceRootInput` with `vfs::FileId` ({id:?}); this is a bug" ,) , } ; * file_source_root } pub fn set_file_source_root_with_durability (& self , db : & mut dyn SourceDatabase , id : vfs :: FileId , source_root_id : SourceRootId , durability : Durability ,) { match self . file_source_roots . entry (id) { Entry :: Occupied (mut occupied) => { occupied . get_mut () . set_source_root_id (db) . with_durability (durability) . to (source_root_id) ; } Entry :: Vacant (vacant) => { let file_source_root = FileSourceRootInput :: builder (source_root_id) . durability (durability) . new (db) ; vacant . insert (file_source_root) ; } } ; } }
    };
}

impl_81!()