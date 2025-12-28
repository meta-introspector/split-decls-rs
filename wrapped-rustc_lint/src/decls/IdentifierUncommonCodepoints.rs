macro_rules! IdentifierUncommonCodepoints {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_identifier_uncommon_codepoints)] # [note] pub (crate) struct IdentifierUncommonCodepoints { pub codepoints : Vec < char > , pub codepoints_len : usize , pub identifier_type : & 'static str , }
    };
}

IdentifierUncommonCodepoints!()