macro_rules! CoverageSpan {
    () => {
        # [doc = " A span of source code coordinates to be embedded in coverage metadata."] # [doc = ""] # [doc = " Must match the layout of `LLVMRustCoverageSpan`."] # [derive (Clone , Debug)] # [repr (C)] pub (crate) struct CoverageSpan { # [doc = " Local index into the function's local-to-global file ID table."] # [doc = " The value at that index is itself an index into the coverage filename"] # [doc = " table in the CGU's `__llvm_covmap` section."] pub (crate) file_id : u32 , # [doc = " 1-based starting line of the source code span."] pub (crate) start_line : u32 , # [doc = " 1-based starting column of the source code span."] pub (crate) start_col : u32 , # [doc = " 1-based ending line of the source code span."] pub (crate) end_line : u32 , # [doc = " 1-based ending column of the source code span. High bit must be unset."] pub (crate) end_col : u32 , }
    };
}

CoverageSpan!()