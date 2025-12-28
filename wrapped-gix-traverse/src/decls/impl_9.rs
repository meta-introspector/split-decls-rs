macro_rules! deps {
    () => {
        CommitState!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl CommitState { pub fn is_hidden (& self) -> bool { matches ! (self , CommitState :: Hidden) } pub fn is_interesting (& self) -> bool { matches ! (self , CommitState :: Interesting) } }
    };
}

impl_9!();