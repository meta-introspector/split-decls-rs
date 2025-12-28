macro_rules! VirtualFileMapping {
    () => {
        # [doc = " Holds a mapping from \"local\" (per-function) file IDs to their corresponding"] # [doc = " source files."] # [derive (Debug , Default)] struct VirtualFileMapping { local_file_table : IndexVec < LocalFileId , Arc < SourceFile > > , }
    };
}

VirtualFileMapping!()