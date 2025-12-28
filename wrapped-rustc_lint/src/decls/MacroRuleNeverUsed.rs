macro_rules! MacroRuleNeverUsed {
    () => {
        # [derive (LintDiagnostic)] # [diag (lint_macro_rule_never_used)] pub (crate) struct MacroRuleNeverUsed { pub n : usize , pub name : Symbol , }
    };
}

MacroRuleNeverUsed!();