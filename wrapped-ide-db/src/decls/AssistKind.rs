macro_rules! AssistKind {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum AssistKind { QuickFix , Generate , Refactor , RefactorExtract , RefactorInline , RefactorRewrite , }
    };
}

AssistKind!()