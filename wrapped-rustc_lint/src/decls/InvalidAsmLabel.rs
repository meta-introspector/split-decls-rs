macro_rules! InvalidAsmLabel {
    () => {
        # [derive (LintDiagnostic)] pub (crate) enum InvalidAsmLabel { # [diag (lint_invalid_asm_label_named)] # [help] # [note] Named { # [note (lint_invalid_asm_label_no_span)] missing_precise_span : bool , } , # [diag (lint_invalid_asm_label_format_arg)] # [help] # [note (lint_note1)] # [note (lint_note2)] FormatArg { # [note (lint_invalid_asm_label_no_span)] missing_precise_span : bool , } , # [diag (lint_invalid_asm_label_binary)] # [help] # [note (lint_note1)] # [note (lint_note2)] Binary { # [note (lint_invalid_asm_label_no_span)] missing_precise_span : bool , # [label] span : Span , } , }
    };
}

InvalidAsmLabel!();