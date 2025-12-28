macro_rules! deps {
    () => {
        Binding!();
        DiffLineType!();
    };
}

macro_rules! impl_343 {
    () => {
        deps!();
        impl Binding for DiffLineType { type Raw = raw :: git_diff_line_t ; unsafe fn from_raw (raw : raw :: git_diff_line_t) -> Self { match raw { raw :: GIT_DIFF_LINE_CONTEXT => DiffLineType :: Context , raw :: GIT_DIFF_LINE_ADDITION => DiffLineType :: Addition , raw :: GIT_DIFF_LINE_DELETION => DiffLineType :: Deletion , raw :: GIT_DIFF_LINE_CONTEXT_EOFNL => DiffLineType :: ContextEOFNL , raw :: GIT_DIFF_LINE_ADD_EOFNL => DiffLineType :: AddEOFNL , raw :: GIT_DIFF_LINE_DEL_EOFNL => DiffLineType :: DeleteEOFNL , raw :: GIT_DIFF_LINE_FILE_HDR => DiffLineType :: FileHeader , raw :: GIT_DIFF_LINE_HUNK_HDR => DiffLineType :: HunkHeader , raw :: GIT_DIFF_LINE_BINARY => DiffLineType :: Binary , _ => panic ! ("Unknown git diff line type") , } } fn raw (& self) -> raw :: git_diff_line_t { match * self { DiffLineType :: Context => raw :: GIT_DIFF_LINE_CONTEXT , DiffLineType :: Addition => raw :: GIT_DIFF_LINE_ADDITION , DiffLineType :: Deletion => raw :: GIT_DIFF_LINE_DELETION , DiffLineType :: ContextEOFNL => raw :: GIT_DIFF_LINE_CONTEXT_EOFNL , DiffLineType :: AddEOFNL => raw :: GIT_DIFF_LINE_ADD_EOFNL , DiffLineType :: DeleteEOFNL => raw :: GIT_DIFF_LINE_DEL_EOFNL , DiffLineType :: FileHeader => raw :: GIT_DIFF_LINE_FILE_HDR , DiffLineType :: HunkHeader => raw :: GIT_DIFF_LINE_HUNK_HDR , DiffLineType :: Binary => raw :: GIT_DIFF_LINE_BINARY , } } }
    };
}

impl_343!();