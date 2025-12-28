macro_rules! deps {
    () => {
        BodyId!();
    };
}

macro_rules! AnonConst {
    () => {
        deps!();
        # [doc = " A constant (expression) that's not an item or associated item,"] # [doc = " but needs its own `DefId` for type-checking, const-eval, etc."] # [doc = " These are usually found nested inside types (e.g., array lengths)"] # [doc = " or expressions (e.g., repeat counts), and also used to define"] # [doc = " explicit discriminant values for enum variants."] # [doc = ""] # [doc = " You can check if this anon const is a default in a const param"] # [doc = " `const N: usize = { ... }` with `tcx.hir_opt_const_param_default_param_def_id(..)`"] # [derive (Copy , Clone , Debug , HashStable_Generic)] pub struct AnonConst { # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub body : BodyId , pub span : Span , }
    };
}

AnonConst!()