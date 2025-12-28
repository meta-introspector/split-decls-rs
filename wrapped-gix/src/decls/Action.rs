macro_rules! deps {
    () => {
        Clone!();
        Default!();
    };
}

macro_rules! Action {
    () => {
        deps!();
        # [doc = " Returned by the `for_each` function to control flow."] # [derive (Default , Clone , Copy , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Action { # [doc = " Continue the traversal of changes."] # [default] Continue , # [doc = " Stop the traversal of changes and stop calling this function."] Cancel , }
    };
}

Action!()