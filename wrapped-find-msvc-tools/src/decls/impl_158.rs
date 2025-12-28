macro_rules! deps {
    () => {
        VsInstances!();
        VsInstance!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl IntoIterator for VsInstances { type Item = VsInstance ; # [allow (bare_trait_objects)] type IntoIter = Box < Iterator < Item = Self :: Item > > ; fn into_iter (self) -> Self :: IntoIter { match self { VsInstances :: ComBased (e) => { Box :: new (e . into_iter () . filter_map (Result :: ok) . map (VsInstance :: Com)) } VsInstances :: VswhereBased (v) => Box :: new (std :: iter :: once (VsInstance :: Vswhere (v))) , } } }
    };
}

impl_158!();