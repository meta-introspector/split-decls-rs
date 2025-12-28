macro_rules! deps {
    () => {
        Constness!();
        IsAsync!();
        HeaderSafety!();
    };
}

macro_rules! FnHeader {
    () => {
        deps!();
        # [derive (Copy , Clone , Debug , HashStable_Generic)] pub struct FnHeader { pub safety : HeaderSafety , pub constness : Constness , pub asyncness : IsAsync , pub abi : ExternAbi , }
    };
}

FnHeader!();