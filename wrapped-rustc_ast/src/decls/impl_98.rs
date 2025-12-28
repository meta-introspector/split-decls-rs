macro_rules! deps {
    () => {
        GenBlockKind!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl GenBlockKind { pub fn modifier (& self) -> & 'static str { match self { GenBlockKind :: Async => "async" , GenBlockKind :: Gen => "gen" , GenBlockKind :: AsyncGen => "async gen" , } } }
    };
}

impl_98!()