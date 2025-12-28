macro_rules! deps {
    () => {
        BodyId!();
    };
}

macro_rules! ConstBlock {
    () => {
        deps!();
        # [doc = " An inline constant expression `const { something }`."] # [derive (Copy , Clone , Debug , HashStable_Generic)] pub struct ConstBlock { # [stable_hasher (ignore)] pub hir_id : HirId , pub def_id : LocalDefId , pub body : BodyId , }
    };
}

ConstBlock!();