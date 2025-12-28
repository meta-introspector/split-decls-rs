macro_rules! deps {
    () => {
        EarlyContext!();
    };
}

macro_rules! declare_early_lint_pass {
    () => {
        deps!();
        macro_rules ! declare_early_lint_pass { ([] , [$ ($ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => (pub trait EarlyLintPass : LintPass { $ (# [inline (always)] fn $ name (& mut self , _ : & EarlyContext <'_ >, $ (_ : $ arg) ,*) { }) * }) }
    };
}

declare_early_lint_pass!()