macro_rules! ReturnAction {
    () => {
        # [doc = " Data returned by [`Machine::after_stack_pop`], and consumed by"] # [doc = " [`InterpCx::return_from_current_stack_frame`] to determine what actions should be done when"] # [doc = " returning from a stack frame."] # [derive (Eq , PartialEq , Debug , Copy , Clone)] pub enum ReturnAction { # [doc = " Indicates that no special handling should be"] # [doc = " done - we'll either return normally or unwind"] # [doc = " based on the terminator for the function"] # [doc = " we're leaving."] Normal , # [doc = " Indicates that we should *not* jump to the return/unwind address, as the callback already"] # [doc = " took care of everything."] NoJump , # [doc = " Returned by [`InterpCx::pop_stack_frame_raw`] when no cleanup should be done."] NoCleanup , }
    };
}

ReturnAction!();