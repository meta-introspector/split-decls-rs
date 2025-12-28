macro_rules! deps {
    () => {
        Diagnostic!();
    };
}

macro_rules! UnknownArchiveKind {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (codegen_ssa_unknown_archive_kind)] pub struct UnknownArchiveKind < 'a > { pub kind : & 'a str , }
    };
}

UnknownArchiveKind!()