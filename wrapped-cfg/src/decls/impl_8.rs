macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl IntoIterator for CfgOptions { type Item = < FxHashSet < CfgAtom > as IntoIterator > :: Item ; type IntoIter = < FxHashSet < CfgAtom > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { < FxHashSet < CfgAtom > as IntoIterator > :: into_iter (self . enabled) } }
    };
}

impl_8!()