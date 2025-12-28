macro_rules! deps {
    () => {
        Hir!();
        Repetition!();
        Capture!();
        Concat!();
        Alternation!();
        Literal!();
        Class!();
        HirKind!();
        Look!();
    };
}

macro_rules! impl_222 {
    () => {
        deps!();
        impl HirKind { # [doc = " Returns a slice of this kind's sub-expressions, if any."] pub fn subs (& self) -> & [Hir] { use core :: slice :: from_ref ; match * self { HirKind :: Empty | HirKind :: Literal (_) | HirKind :: Class (_) | HirKind :: Look (_) => & [] , HirKind :: Repetition (Repetition { ref sub , .. }) => from_ref (sub) , HirKind :: Capture (Capture { ref sub , .. }) => from_ref (sub) , HirKind :: Concat (ref subs) => subs , HirKind :: Alternation (ref subs) => subs , } } }
    };
}

impl_222!();