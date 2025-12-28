macro_rules! deps {
    () => {
        State!();
        Statistics!();
        Stack!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        # [doc = " Mutation"] impl Stack { # [doc = " Reset the statistics after returning them."] pub fn take_statistics (& mut self) -> Statistics { std :: mem :: take (& mut self . statistics) } # [doc = " Return our state for applying changes."] pub fn state_mut (& mut self) -> & mut State { & mut self . state } # [doc = " Change the `case` of the next match to the given one."] pub fn set_case (& mut self , case : gix_glob :: pattern :: Case) -> & mut Self { self . case = case ; self } }
    };
}

impl_8!()