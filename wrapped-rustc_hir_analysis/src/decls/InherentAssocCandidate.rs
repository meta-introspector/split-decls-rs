macro_rules! InherentAssocCandidate {
    () => {
        # [derive (Copy , Clone , TypeFoldable , TypeVisitable , Debug)] pub struct InherentAssocCandidate { pub impl_ : DefId , pub assoc_item : DefId , pub scope : DefId , }
    };
}

InherentAssocCandidate!();