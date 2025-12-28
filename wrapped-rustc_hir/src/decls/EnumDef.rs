macro_rules! deps {
    () => {
        Variant!();
    };
}

macro_rules! EnumDef {
    () => {
        deps!();
        # [derive (Debug , Clone , Copy , HashStable_Generic)] pub struct EnumDef < 'hir > { pub variants : & 'hir [Variant < 'hir >] , }
    };
}

EnumDef!();