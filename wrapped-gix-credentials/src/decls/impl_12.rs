macro_rules! deps {
    () => {
        Action!();
        NextAction!();
    };
}

macro_rules! impl_12 {
    () => {
        deps!();
        impl NextAction { # [doc = " Approve the result of the previous [Action] and store for lookup."] pub fn store (self) -> Action { Action :: Store (self . previous_output) } # [doc = " Reject the result of the previous [Action] and erase it as to not be returned when being looked up."] pub fn erase (self) -> Action { Action :: Erase (self . previous_output) } }
    };
}

impl_12!()