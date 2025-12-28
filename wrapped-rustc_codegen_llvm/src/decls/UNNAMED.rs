macro_rules! UNNAMED {
    () => {
        # [doc = " Empty string, to be used where LLVM expects an instruction name, indicating"] # [doc = " that the instruction is to be left unnamed (i.e. numbered, in textual IR)."] pub (crate) const UNNAMED : * const c_char = c"" . as_ptr () ;
    };
}

UNNAMED!();