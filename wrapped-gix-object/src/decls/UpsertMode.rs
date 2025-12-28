macro_rules! UpsertMode {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq)] enum UpsertMode { Normal , # [doc = " Only make sure there is a tree at the given location (requires kind tree and null-id)"] AssureTreeOnly , }
    };
}

UpsertMode!();