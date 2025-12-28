macro_rules! deps {
    () => {
        NextAction!();
    };
}

macro_rules! Outcome {
    () => {
        deps!();
        # [doc = " The outcome of the credentials top-level functions to obtain a complete identity."] # [derive (Debug , Clone , Eq , PartialEq)] pub struct Outcome { # [doc = " The identity provide by the helper."] pub identity : gix_sec :: identity :: Account , # [doc = " A handle to the action to perform next in another call to [`helper::invoke()`][crate::helper::invoke()]."] pub next : helper :: NextAction , }
    };
}

Outcome!();