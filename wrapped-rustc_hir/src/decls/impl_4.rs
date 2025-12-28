macro_rules! deps {
    () => {
        InlineAttr!();
    };
}

macro_rules! impl_4 {
    () => {
        deps!();
        impl InlineAttr { pub fn always (& self) -> bool { match self { InlineAttr :: Always | InlineAttr :: Force { .. } => true , InlineAttr :: None | InlineAttr :: Hint | InlineAttr :: Never => false , } } }
    };
}

impl_4!()