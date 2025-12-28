macro_rules! deps {
    () => {
        TargetKindData!();
        TargetKind!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl From < TargetKindData > for TargetKind { fn from (data : TargetKindData) -> Self { match data { TargetKindData :: Bin => TargetKind :: Bin , TargetKindData :: Lib => TargetKind :: Lib { is_proc_macro : false } , TargetKindData :: Test => TargetKind :: Test , } } }
    };
}

impl_18!()