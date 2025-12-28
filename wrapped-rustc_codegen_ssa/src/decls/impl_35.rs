macro_rules! deps {
    () => {
        UnknownArchiveKind!();
        ArArchiveBuilder!();
        ArchiveEntry!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        impl < 'a > ArArchiveBuilder < 'a > { fn build_inner (self , output : & Path) -> io :: Result < bool > { let archive_kind = match & * self . sess . target . archive_format { "gnu" => ArchiveKind :: Gnu , "bsd" => ArchiveKind :: Bsd , "darwin" => ArchiveKind :: Darwin , "coff" => ArchiveKind :: Coff , "aix_big" => ArchiveKind :: AixBig , kind => { self . sess . dcx () . emit_fatal (UnknownArchiveKind { kind }) ; } } ; let mut entries = Vec :: new () ; for (entry_name , entry) in self . entries { let data = match entry { ArchiveEntry :: FromArchive { archive_index , file_range } => { let src_archive = & self . src_archives [archive_index] ; let data = & src_archive . 1 [file_range . 0 as usize .. file_range . 0 as usize + file_range . 1 as usize] ; Box :: new (data) as Box < dyn AsRef < [u8] > > } ArchiveEntry :: File (file) => unsafe { Box :: new (Mmap :: map (File :: open (file) . map_err (| err | { io_error_context ("failed to open object file" , err) }) ?) . map_err (| err | io_error_context ("failed to map object file" , err)) ? ,) as Box < dyn AsRef < [u8] > > } , } ; entries . push (NewArchiveMember { buf : data , object_reader : self . object_reader , member_name : String :: from_utf8 (entry_name) . unwrap () , mtime : 0 , uid : 0 , gid : 0 , perms : 0o644 , }) } let archive_tmpdir = TempDirBuilder :: new () . suffix (".temp-archive") . tempdir_in (output . parent () . unwrap_or_else (| | Path :: new (""))) . map_err (| err | { io_error_context ("couldn't create a directory for the temp file" , err) }) ? ; let archive_tmpfile_path = archive_tmpdir . path () . join ("tmp.a") ; let archive_tmpfile = File :: create_new (& archive_tmpfile_path) . map_err (| err | io_error_context ("couldn't create the temp file" , err)) ? ; let mut archive_tmpfile = BufWriter :: new (archive_tmpfile) ; write_archive_to_stream (& mut archive_tmpfile , & entries , archive_kind , false , Some (self . sess . target . arch == "arm64ec") ,) ? ; archive_tmpfile . flush () ? ; drop (archive_tmpfile) ; let any_entries = ! entries . is_empty () ; drop (entries) ; drop (self . src_archives) ; fs :: rename (archive_tmpfile_path , output) . map_err (| err | io_error_context ("failed to rename archive file" , err)) ? ; archive_tmpdir . close () . map_err (| err | io_error_context ("failed to remove temporary directory" , err)) ? ; Ok (any_entries) } }
    };
}

impl_35!();