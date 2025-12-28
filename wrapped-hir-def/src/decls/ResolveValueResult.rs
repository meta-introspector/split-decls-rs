macro_rules! deps {
    () => {
        ValueNs!();
        ImportOrGlob!();
        TypeNs!();
        ImportOrExternCrate!();
    };
}

macro_rules! ResolveValueResult {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq , Hash)] pub enum ResolveValueResult { ValueNs (ValueNs , Option < ImportOrGlob >) , Partial (TypeNs , usize , Option < ImportOrExternCrate >) , }
    };
}

ResolveValueResult!()