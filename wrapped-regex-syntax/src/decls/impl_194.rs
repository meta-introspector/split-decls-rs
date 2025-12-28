macro_rules! deps {
    () => {
        ClassUnicode!();
        Flags!();
        Hir!();
        Literal!();
        HirFrame!();
        ClassBytes!();
        Repetition!();
        Group!();
    };
}

macro_rules! impl_194 {
    () => {
        deps!();
        impl HirFrame { # [doc = " Assert that the current stack frame is an Hir expression and return it."] fn unwrap_expr (self) -> Hir { match self { HirFrame :: Expr (expr) => expr , HirFrame :: Literal (lit) => Hir :: literal (lit) , _ => panic ! ("tried to unwrap expr from HirFrame, got: {self:?}") , } } # [doc = " Assert that the current stack frame is a Unicode class expression and"] # [doc = " return it."] fn unwrap_class_unicode (self) -> hir :: ClassUnicode { match self { HirFrame :: ClassUnicode (cls) => cls , _ => panic ! ("tried to unwrap Unicode class \
                 from HirFrame, got: {:?}" , self) , } } # [doc = " Assert that the current stack frame is a byte class expression and"] # [doc = " return it."] fn unwrap_class_bytes (self) -> hir :: ClassBytes { match self { HirFrame :: ClassBytes (cls) => cls , _ => panic ! ("tried to unwrap byte class \
                 from HirFrame, got: {:?}" , self) , } } # [doc = " Assert that the current stack frame is a repetition sentinel. If it"] # [doc = " isn't, then panic."] fn unwrap_repetition (self) { match self { HirFrame :: Repetition => { } _ => { panic ! ("tried to unwrap repetition from HirFrame, got: {self:?}") } } } # [doc = " Assert that the current stack frame is a group indicator and return"] # [doc = " its corresponding flags (the flags that were active at the time the"] # [doc = " group was entered)."] fn unwrap_group (self) -> Flags { match self { HirFrame :: Group { old_flags } => old_flags , _ => { panic ! ("tried to unwrap group from HirFrame, got: {self:?}") } } } # [doc = " Assert that the current stack frame is an alternation pipe sentinel. If"] # [doc = " it isn't, then panic."] fn unwrap_alternation_pipe (self) { match self { HirFrame :: AlternationBranch => { } _ => { panic ! ("tried to unwrap alt pipe from HirFrame, got: {self:?}") } } } }
    };
}

impl_194!();