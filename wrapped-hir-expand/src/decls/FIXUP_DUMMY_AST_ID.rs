macro_rules! FIXUP_DUMMY_AST_ID {
    () => {
        const FIXUP_DUMMY_AST_ID : ErasedFileAstId = FIXUP_ERASED_FILE_AST_ID_MARKER ;
    };
}

FIXUP_DUMMY_AST_ID!();