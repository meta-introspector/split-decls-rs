macro_rules! deps {
    () => {
        AutorefOrPtrAdjustment!();
    };
}

macro_rules! impl_258 {
    () => {
        deps!();
        impl AutorefOrPtrAdjustment { fn get_unsize (& self) -> bool { match self { AutorefOrPtrAdjustment :: Autoref { mutbl : _ , unsize } => * unsize , AutorefOrPtrAdjustment :: ToConstPtr => false , AutorefOrPtrAdjustment :: ReborrowPin (_) => false , } } }
    };
}

impl_258!()