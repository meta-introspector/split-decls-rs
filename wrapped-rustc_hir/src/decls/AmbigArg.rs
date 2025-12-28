macro_rules! deps {
    () => {
        Ty!();
        ConstArg!();
    };
}

macro_rules! AmbigArg {
    () => {
        deps!();
        # [doc = " An uninhabited enum used to make `Infer` variants on [`Ty`] and [`ConstArg`] be"] # [doc = " unreachable. Zero-Variant enums are guaranteed to have the same layout as the never"] # [doc = " type."] # [derive (Debug , Clone , Copy , HashStable_Generic)] pub enum AmbigArg { }
    };
}

AmbigArg!()