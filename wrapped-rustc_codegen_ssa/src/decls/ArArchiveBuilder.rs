macro_rules! deps {
    () => {
        ArchiveEntry!();
    };
}

macro_rules! ArArchiveBuilder {
    () => {
        deps!();
        # [must_use = "must call build() to finish building the archive"] pub struct ArArchiveBuilder < 'a > { sess : & 'a Session , object_reader : & 'static ObjectReader , src_archives : Vec < (PathBuf , Mmap) > , entries : Vec < (Vec < u8 > , ArchiveEntry) > , }
    };
}

ArArchiveBuilder!()