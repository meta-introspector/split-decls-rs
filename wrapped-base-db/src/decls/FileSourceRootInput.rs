macro_rules! deps {
    () => {
        SourceRootId!();
    };
}

macro_rules! FileSourceRootInput {
    () => {
        deps!();
        # [salsa_macros :: input (debug)] pub struct FileSourceRootInput { pub source_root_id : SourceRootId , }
    };
}

FileSourceRootInput!()