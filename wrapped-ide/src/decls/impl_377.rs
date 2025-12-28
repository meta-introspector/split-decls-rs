macro_rules! deps {
    () => {
        RunnableKind!();
        RunnableDiscKind!();
    };
}

macro_rules! impl_377 {
    () => {
        deps!();
        impl RunnableKind { fn disc (& self) -> RunnableDiscKind { match self { RunnableKind :: TestMod { .. } => RunnableDiscKind :: TestMod , RunnableKind :: Test { .. } => RunnableDiscKind :: Test , RunnableKind :: DocTest { .. } => RunnableDiscKind :: DocTest , RunnableKind :: Bench { .. } => RunnableDiscKind :: Bench , RunnableKind :: Bin => RunnableDiscKind :: Bin , } } }
    };
}

impl_377!();