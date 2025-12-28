macro_rules! deps {
    () => {
        FileSourceRootInput!();
        FileText!();
        SourceRootInput!();
        SourceRootId!();
    };
}

macro_rules! Files {
    () => {
        deps!();
        # [derive (Debug , Default)] pub struct Files { files : Arc < DashMap < vfs :: FileId , FileText , BuildHasherDefault < FxHasher > > > , source_roots : Arc < DashMap < SourceRootId , SourceRootInput , BuildHasherDefault < FxHasher > > > , file_source_roots : Arc < DashMap < vfs :: FileId , FileSourceRootInput , BuildHasherDefault < FxHasher > > > , }
    };
}

Files!()