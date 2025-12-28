macro_rules! deps {
    () => {
        ArchiveBuildFailure!();
        ArchiveEntry!();
        ArchiveBuilder!();
        ArArchiveBuilder!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl < 'a > ArchiveBuilder for ArArchiveBuilder < 'a > { fn add_archive (& mut self , archive_path : & Path , mut skip : Box < dyn FnMut (& str) -> bool + 'static > ,) -> io :: Result < () > { let mut archive_path = archive_path . to_path_buf () ; if self . sess . target . llvm_target . contains ("-apple-macosx") && let Some (new_archive_path) = try_extract_macho_fat_archive (self . sess , & archive_path) ? { archive_path = new_archive_path } if self . src_archives . iter () . any (| archive | archive . 0 == archive_path) { return Ok (()) ; } let archive_map = unsafe { Mmap :: map (File :: open (& archive_path) ?) ? } ; let archive = ArchiveFile :: parse (& * archive_map) . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; let archive_index = self . src_archives . len () ; for entry in archive . members () { let entry = entry . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; let file_name = String :: from_utf8 (entry . name () . to_vec ()) . map_err (| err | io :: Error :: new (io :: ErrorKind :: InvalidData , err)) ? ; if ! skip (& file_name) { if entry . is_thin () { let member_path = archive_path . parent () . unwrap () . join (Path :: new (& file_name)) ; self . entries . push ((file_name . into_bytes () , ArchiveEntry :: File (member_path))) ; } else { self . entries . push ((file_name . into_bytes () , ArchiveEntry :: FromArchive { archive_index , file_range : entry . file_range () } ,)) ; } } } self . src_archives . push ((archive_path , archive_map)) ; Ok (()) } # [doc = " Adds an arbitrary file to this archive"] fn add_file (& mut self , file : & Path) { self . entries . push ((file . file_name () . unwrap () . to_str () . unwrap () . to_string () . into_bytes () , ArchiveEntry :: File (file . to_owned ()) ,)) ; } # [doc = " Combine the provided files, rlibs, and native libraries into a single"] # [doc = " `Archive`."] fn build (self : Box < Self > , output : & Path) -> bool { let sess = self . sess ; match self . build_inner (output) { Ok (any_members) => any_members , Err (error) => { sess . dcx () . emit_fatal (ArchiveBuildFailure { path : output . to_owned () , error }) } } } }
    };
}

impl_34!()