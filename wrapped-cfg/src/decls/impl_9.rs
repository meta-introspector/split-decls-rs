macro_rules! deps {
    () => {
        CfgOptions!();
    };
}

macro_rules! impl_9 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a CfgOptions { type Item = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: Item ; type IntoIter = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { < & FxHashSet < CfgAtom > as IntoIterator > :: into_iter (& self . enabled) } }
    };
}

impl_9!()