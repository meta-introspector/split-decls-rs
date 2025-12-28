macro_rules! deps {
    () => {
        LateContext!();
    };
}

macro_rules! declare_late_lint_pass {
    () => {
        deps!();
        # [doc = " Trait for types providing lint checks."] # [doc = ""] # [doc = " Each `check` method checks a single syntax node, and should not"] # [doc = " invoke methods recursively (unlike `Visitor`). By default they"] # [doc = " do nothing."] macro_rules ! declare_late_lint_pass { ([] , [$ ($ (# [$ attr : meta]) * fn $ name : ident ($ ($ param : ident : $ arg : ty) ,*) ;) *]) => (pub trait LateLintPass <'tcx >: LintPass { $ (# [inline (always)] fn $ name (& mut self , _ : & LateContext <'tcx >, $ (_ : $ arg) ,*) { }) * }) }
    };
}

declare_late_lint_pass!()