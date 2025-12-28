macro_rules! deps {
    () => {
        Process!();
        ToGitOutcome!();
    };
}

macro_rules! impl_109 {
    () => {
        deps!();
        impl < R > std :: io :: Read for ToGitOutcome < '_ , R > where R : std :: io :: Read , { fn read (& mut self , buf : & mut [u8]) -> std :: io :: Result < usize > { match self { ToGitOutcome :: Unchanged (r) => r . read (buf) , ToGitOutcome :: Process (r) => r . read (buf) , ToGitOutcome :: Buffer (r) => r . read (buf) , } } }
    };
}

impl_109!();