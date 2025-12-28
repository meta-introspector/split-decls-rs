macro_rules! InferArg {
    () => {
        # [derive (Clone , Copy , Debug , HashStable_Generic)] pub struct InferArg { # [stable_hasher (ignore)] pub hir_id : HirId , pub span : Span , }
    };
}

InferArg!();