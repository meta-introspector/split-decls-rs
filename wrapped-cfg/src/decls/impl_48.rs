macro_rules! deps {
    () => {
        CfgAtom!();
        CfgOptions!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < 'a > IntoIterator for & 'a CfgOptions { type Item = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: Item ; type IntoIter = < & 'a FxHashSet < CfgAtom > as IntoIterator > :: IntoIter ; fn into_iter (self) -> Self :: IntoIter { < & FxHashSet < CfgAtom > as IntoIterator > :: into_iter (& self . enabled) } }
    };
}

impl_48!();