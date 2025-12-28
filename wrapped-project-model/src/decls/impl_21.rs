macro_rules! deps {
    () => {
        RunnableKindData!();
        RunnableKind!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < RunnableKindData > for RunnableKind { fn from (data : RunnableKindData) -> Self { match data { RunnableKindData :: Check => RunnableKind :: Check , RunnableKindData :: Run => RunnableKind :: Run , RunnableKindData :: TestOne => RunnableKind :: TestOne , } } }
    };
}

impl_21!();