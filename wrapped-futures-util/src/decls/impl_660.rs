macro_rules! impl_660 {
    () => {
        impl < St : TryStream , C : Default > TryCollect < St , C > { pub (super) fn new (s : St) -> Self { Self { stream : s , items : Default :: default () } } }
    };
}

impl_660!();