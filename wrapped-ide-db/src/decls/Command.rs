macro_rules! Command {
    () => {
        # [derive (Debug , Clone , Copy , PartialEq , Eq)] pub enum Command { # [doc = " Show the parameter hints popup."] TriggerParameterHints , # [doc = " Rename the just inserted item."] Rename , }
    };
}

Command!()