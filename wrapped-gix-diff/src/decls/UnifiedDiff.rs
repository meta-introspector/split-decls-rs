macro_rules! deps {
    () => {
        ConsumeHunk!();
        DiffLineKind!();
        Error!();
    };
}

macro_rules! UnifiedDiff {
    () => {
        deps!();
        # [doc = " A [`Sink`] that creates a unified diff. It can be used to create a textual diff in the"] # [doc = " format typically output by `git` or `gnu-diff` if the `-u` option is used."] pub struct UnifiedDiff < 'a , T , D > where T : Hash + Eq + AsRef < [u8] > , D : ConsumeHunk , { before : & 'a [Token] , after : & 'a [Token] , interner : & 'a Interner < T > , # [doc = " The 0-based start position in the 'before' tokens for the accumulated hunk for display in the header."] before_hunk_start : u32 , # [doc = " The size of the accumulated 'before' hunk in lines for display in the header."] before_hunk_len : u32 , # [doc = " The 0-based start position in the 'after' tokens for the accumulated hunk for display in the header."] after_hunk_start : u32 , # [doc = " The size of the accumulated 'after' hunk in lines."] after_hunk_len : u32 , ctx_pos : Option < u32 > , # [doc = " Symmetrical context before and after the changed hunk."] ctx_size : u32 , buffer : Vec < (DiffLineKind , & 'a [u8]) > , delegate : D , err : Option < std :: io :: Error > , }
    };
}

UnifiedDiff!()