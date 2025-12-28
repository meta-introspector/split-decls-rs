macro_rules! deps {
    () => {
        Process!();
        ToGitOutcome!();
    };
}

macro_rules! impl_110 {
    () => {
        deps!();
        impl < 'a , R > ToGitOutcome < 'a , R > where R : std :: io :: Read , { # [doc = " If we contain a buffer, and not a stream, return it."] pub fn as_bytes (& self) -> Option < & 'a [u8] > { match self { ToGitOutcome :: Unchanged (_) | ToGitOutcome :: Process (_) => None , ToGitOutcome :: Buffer (b) => Some (b) , } } # [doc = " Return a stream to read the drivers output from. This is only possible if there is only a driver, and no other filter."] pub fn as_read (& mut self) -> Option < & mut (dyn std :: io :: Read + '_) > { match self { ToGitOutcome :: Process (read) => Some (read) , ToGitOutcome :: Unchanged (read) => Some (read) , ToGitOutcome :: Buffer (_) => None , } } # [doc = " Returns `true` if the input buffer was actually changed, or `false` if it is returned directly."] pub fn is_changed (& self) -> bool { ! matches ! (self , ToGitOutcome :: Unchanged (_)) } }
    };
}

impl_110!();