macro_rules! InitializationRequiringAction {
    () => {
        # [derive (Copy , Clone , Debug)] enum InitializationRequiringAction { Borrow , MatchOn , Use , Assignment , PartialAssignment , }
    };
}

InitializationRequiringAction!()