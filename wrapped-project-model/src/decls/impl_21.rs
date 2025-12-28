macro_rules! deps {
    () => {
        RunnableKind!();
        RunnableKindData!();
    };
}

macro_rules! impl_21 {
    () => {
        deps!();
        impl From < RunnableKindData > for RunnableKind { fn from (data : RunnableKindData) -> Self { match data { RunnableKindData :: Check => RunnableKind :: Check , RunnableKindData :: Run => RunnableKind :: Run , RunnableKindData :: TestOne => RunnableKind :: TestOne , } } }
    };
}

impl_21!()