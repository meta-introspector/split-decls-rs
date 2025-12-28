macro_rules! deps {
    () => {
        User!();
        Clone!();
    };
}

macro_rules! Allow {
    () => {
        deps!();
        # [doc = " All allowed values of the `protocol.allow` key."] # [derive (Debug , Clone , Copy , Eq , PartialEq)] pub enum Allow { # [doc = " Allow use this protocol."] Always , # [doc = " Forbid using this protocol"] Never , # [doc = " Only supported if the `GIT_PROTOCOL_FROM_USER` is unset or is set to `1`."] User , }
    };
}

Allow!();