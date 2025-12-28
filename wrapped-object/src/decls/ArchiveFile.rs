macro_rules! deps {
    () => {
        ReadRef!();
        ArchiveKind!();
        Members!();
    };
}

macro_rules! ArchiveFile {
    () => {
        deps!();
        # [doc = " A partially parsed archive file."] # [derive (Debug , Clone , Copy)] pub struct ArchiveFile < 'data , R : ReadRef < 'data > = & 'data [u8] > { data : R , kind : ArchiveKind , members : Members < 'data > , symbols : (u64 , u64) , names : & 'data [u8] , thin : bool , }
    };
}

ArchiveFile!()