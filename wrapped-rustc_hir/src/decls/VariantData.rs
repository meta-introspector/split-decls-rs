macro_rules! deps {
    () => {
        FieldDef!();
    };
}

macro_rules! VariantData {
    () => {
        deps!();
        # [doc = " Fields and constructor IDs of enum variants and structs."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum VariantData < 'hir > { # [doc = " A struct variant."] # [doc = ""] # [doc = " E.g., `Bar { .. }` as in `enum Foo { Bar { .. } }`."] Struct { fields : & 'hir [FieldDef < 'hir >] , recovered : ast :: Recovered } , # [doc = " A tuple variant."] # [doc = ""] # [doc = " E.g., `Bar(..)` as in `enum Foo { Bar(..) }`."] Tuple (& 'hir [FieldDef < 'hir >] , # [stable_hasher (ignore)] HirId , LocalDefId) , # [doc = " A unit variant."] # [doc = ""] # [doc = " E.g., `Bar = ..` as in `enum Foo { Bar = .. }`."] Unit (# [stable_hasher (ignore)] HirId , LocalDefId) , }
    };
}

VariantData!();