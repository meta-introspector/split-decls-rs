macro_rules! deps {
    () => {
        TreeIter!();
        TreeEntry!();
        Tree!();
    };
}

macro_rules! impl_826 {
    () => {
        deps!();
        impl < 'repo , 'iter > IntoIterator for & 'iter Tree < 'repo > { type Item = TreeEntry < 'iter > ; type IntoIter = TreeIter < 'iter > ; fn into_iter (self) -> Self :: IntoIter { self . iter () } }
    };
}

impl_826!();