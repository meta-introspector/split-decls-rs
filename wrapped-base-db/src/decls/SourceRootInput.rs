macro_rules! SourceRootInput {
    () => {
        # [salsa_macros :: input (debug)] pub struct SourceRootInput { pub source_root : Arc < SourceRoot > , }
    };
}

SourceRootInput!()