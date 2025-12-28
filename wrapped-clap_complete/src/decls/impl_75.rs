macro_rules! deps {
    () => {
        CompletionCandidate!();
    };
}

macro_rules! impl_75 {
    () => {
        deps!();
        impl CompletionCandidate { # [doc = " Create a new completion candidate"] pub fn new (value : impl Into < OsString >) -> Self { let value = value . into () ; Self { value , .. Default :: default () } } # [doc = " Set the help message of the completion candidate"] pub fn help (mut self , help : Option < StyledStr >) -> Self { self . help = help ; self } # [doc = " Only first for a given Id is shown"] # [doc = ""] # [doc = " To reduce the risk of conflicts, this should likely contain a namespace."] pub fn id (mut self , id : Option < String >) -> Self { self . id = id ; self } # [doc = " Group candidates by tag"] # [doc = ""] # [doc = " Future: these may become user-visible"] pub fn tag (mut self , tag : Option < StyledStr >) -> Self { self . tag = tag ; self } # [doc = " Sort weight within a [`CompletionCandidate::tag`]"] pub fn display_order (mut self , order : Option < usize >) -> Self { self . display_order = order ; self } # [doc = " Set the visibility of the completion candidate"] # [doc = ""] # [doc = " Only shown when there is no visible candidate for completing the current argument."] pub fn hide (mut self , hidden : bool) -> Self { self . hidden = hidden ; self } # [doc = " Add a prefix to the value of completion candidate"] # [doc = ""] # [doc = " This is generally used for post-process by [`complete`][crate::engine::complete()] for"] # [doc = " things like pre-pending flags, merging delimiter-separated values, etc."] pub fn add_prefix (mut self , prefix : impl Into < OsString >) -> Self { let suffix = self . value ; let mut value = prefix . into () ; value . push (& suffix) ; self . value = value ; self } }
    };
}

impl_75!();