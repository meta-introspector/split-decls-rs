macro_rules! InherentImpls {
    () => {
        # [derive (Debug , PartialEq , Eq)] pub struct InherentImpls { map : FxHashMap < SimplifiedType , Box < [ImplId] > > , }
    };
}

InherentImpls!();