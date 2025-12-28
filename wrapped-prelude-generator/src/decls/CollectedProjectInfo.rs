macro_rules! deps {
    () => {
        Declaration!();
    };
}

macro_rules! CollectedProjectInfo {
    () => {
        deps!();
        # [derive (Serialize , Deserialize , Debug)] pub struct CollectedProjectInfo { pub declarations : Vec < Declaration > , pub types : HashMap < String , split_expanded_lib :: ResolvedDependency > , pub modules : HashMap < String , split_expanded_lib :: ResolvedDependency > , pub crates : HashMap < String , split_expanded_lib :: ResolvedDependency > , }
    };
}

CollectedProjectInfo!()