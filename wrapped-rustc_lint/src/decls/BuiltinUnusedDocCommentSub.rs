macro_rules! BuiltinUnusedDocCommentSub {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum BuiltinUnusedDocCommentSub { # [help (lint_plain_help)] PlainHelp , # [help (lint_block_help)] BlockHelp , }
    };
}

BuiltinUnusedDocCommentSub!();