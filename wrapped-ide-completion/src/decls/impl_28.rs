macro_rules! deps {
    () => {
        Completions!();
        CompletionItem!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl From < Completions > for Vec < CompletionItem > { fn from (val : Completions) -> Self { val . buf } }
    };
}

impl_28!()