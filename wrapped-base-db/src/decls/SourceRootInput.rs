macro_rules! deps {
    () => {
        SourceRoot!();
    };
}

macro_rules! SourceRootInput {
    () => {
        deps!();
        # [salsa_macros :: input (debug)] pub struct SourceRootInput { pub source_root : Arc < SourceRoot > , }
    };
}

SourceRootInput!()