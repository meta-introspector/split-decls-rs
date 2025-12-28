macro_rules! FnKind {
    () => {
        # [derive (Clone , Copy , Debug , PartialEq)] enum FnKind { Free , AssocInherentImpl , AssocTrait , AssocTraitImpl , }
    };
}

FnKind!()