macro_rules! deps {
    () => {
        ToWorktreeOutcome!();
        Process!();
        MaybeDelayed!();
    };
}

macro_rules! impl_107 {
    () => {
        deps!();
        impl ToWorktreeOutcome < '_ , '_ > { # [doc = " Return true if this outcome is delayed. In that case, one isn't allowed to use [`Read`] or cause a panic."] pub fn is_delayed (& self) -> bool { matches ! (self , ToWorktreeOutcome :: Process (driver :: apply :: MaybeDelayed :: Delayed (_))) } # [doc = " Returns `true` if the input buffer was actually changed, or `false` if it is returned directly."] pub fn is_changed (& self) -> bool { ! matches ! (self , ToWorktreeOutcome :: Unchanged (_)) } # [doc = " Return a buffer if we contain one, or `None` otherwise."] # [doc = ""] # [doc = " This method is useful only if it's clear that no driver is available, which may cause a stream to be returned and not a buffer."] pub fn as_bytes (& self) -> Option < & [u8] > { match self { ToWorktreeOutcome :: Unchanged (b) | ToWorktreeOutcome :: Buffer (b) => Some (b) , ToWorktreeOutcome :: Process (_) => None , } } # [doc = " Return a stream to read the drivers output from, if possible."] # [doc = ""] # [doc = " Note that this is only the case if the driver process was applied last *and* didn't delay its output."] pub fn as_read (& mut self) -> Option < & mut (dyn std :: io :: Read + '_) > { match self { ToWorktreeOutcome :: Process (driver :: apply :: MaybeDelayed :: Delayed (_)) | ToWorktreeOutcome :: Unchanged (_) | ToWorktreeOutcome :: Buffer (_) => None , ToWorktreeOutcome :: Process (driver :: apply :: MaybeDelayed :: Immediate (read)) => Some (read) , } } }
    };
}

impl_107!();