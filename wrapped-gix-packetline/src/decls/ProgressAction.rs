macro_rules! ProgressAction {
    () => {
        # [doc = " Allow the read-progress handler to determine how to continue."] # [derive (Copy , Clone , Debug , Eq , PartialEq)] pub enum ProgressAction { # [doc = " Continue reading the next progress if available."] Continue , # [doc = " Abort all IO even if more would be available, claiming the operation was interrupted."] Interrupt , }
    };
}

ProgressAction!();