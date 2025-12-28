macro_rules! deps {
    () => {
        LogChange!();
        RefLog!();
        PreviousValue!();
        Target!();
    };
}

macro_rules! Change {
    () => {
        deps!();
        # [doc = " A description of an edit to perform."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone)] pub enum Change { # [doc = " If previous is not `None`, the ref must exist and its `oid` must agree with the `previous`, and"] # [doc = " we function like `update`."] # [doc = " Otherwise it functions as `create-or-update`."] Update { # [doc = " The desired change to the reference log."] log : LogChange , # [doc = " The expected value already present in the reference."] # [doc = " If a ref was existing previously this field will be overwritten with `MustExistAndMatch(actual_value)` for use after"] # [doc = " the transaction was committed successfully."] expected : PreviousValue , # [doc = " The new state of the reference, either for updating an existing one or creating a new one."] new : Target , } , # [doc = " Delete a reference and optionally check if `previous` is its content."] Delete { # [doc = " The expected value of the reference, with the `MustNotExist` variant being invalid."] # [doc = ""] # [doc = " If a previous ref existed, this value will be filled in automatically as `MustExistAndMatch(actual_value)` and"] # [doc = " can be accessed if the transaction was committed successfully."] expected : PreviousValue , # [doc = " How to treat the reference log during deletion."] log : RefLog , } , }
    };
}

Change!()