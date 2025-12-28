macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! ConflictMapping {
    () => {
        deps!();
        # [doc = " A utility to help define which side is what in the [`Conflict`] type."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] enum ConflictMapping { # [doc = " The sides are as described in the field documentation, i.e. `ours` is `ours`."] Original , # [doc = " The sides are the opposite of the field documentation. i.e. `ours` is `theirs` and `theirs` is `ours`."] Swapped , }
    };
}

ConflictMapping!();