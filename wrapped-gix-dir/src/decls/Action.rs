macro_rules! Action {
    () => {
        # [doc = " A type returned by the [`Delegate::emit()`] as passed to [`walk()`](function::walk())."] # [derive (Debug , Copy , Clone , Eq , PartialEq)] # [must_use] pub enum Action { # [doc = " Continue the traversal as normal."] Continue , # [doc = " Do not continue the traversal, but exit it."] Cancel , }
    };
}

Action!();