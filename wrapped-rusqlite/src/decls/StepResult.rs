macro_rules! StepResult {
    () => {
        # [doc = " Possible successful results of calling"] # [doc = " [`Backup::step`]."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] # [non_exhaustive] pub enum StepResult { # [doc = " The backup is complete."] Done , # [doc = " The step was successful but there are still more pages that need to be"] # [doc = " backed up."] More , # [doc = " The step failed because appropriate locks could not be acquired. This is"] # [doc = " not a fatal error - the step can be retried."] Busy , # [doc = " The step failed because the source connection was writing to the"] # [doc = " database. This is not a fatal error - the step can be retried."] Locked , }
    };
}

StepResult!();