macro_rules! deps {
    () => {
        State!();
        Options!();
        Pipeline!();
        Context!();
    };
}

macro_rules! impl_100 {
    () => {
        deps!();
        # [doc = " Access"] impl Pipeline { # [doc = " Return a mutable reference to the state that handles long running processes."] # [doc = " Interacting with it directly allows to handle delayed results."] pub fn driver_state_mut (& mut self) -> & mut driver :: State { & mut self . processes } # [doc = " Provide mutable context that is made available to the process filters."] # [doc = ""] # [doc = " The context set here is relevant for the [`convert_to_git()`][Self::convert_to_git()] and"] # [doc = " [`convert_to_worktree()`][Self::convert_to_worktree()] methods."] pub fn driver_context_mut (& mut self) -> & mut Context { & mut self . context } # [doc = " Return a set of options for configuration after instantiation."] pub fn options_mut (& mut self) -> & mut Options { & mut self . options } # [doc = " Return our double-buffers for reuse by the caller."] pub fn buffers_mut (& mut self) -> & mut gix_utils :: Buffers { & mut self . bufs } }
    };
}

impl_100!()