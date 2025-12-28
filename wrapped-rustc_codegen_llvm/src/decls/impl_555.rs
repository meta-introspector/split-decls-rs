macro_rules! deps {
    () => {
        LLVMFeature!();
    };
}

macro_rules! impl_555 {
    () => {
        deps!();
        impl < 'a > IntoIterator for LLVMFeature < 'a > { type Item = & 'a str ; type IntoIter = impl Iterator < Item = & 'a str > ; fn into_iter (self) -> Self :: IntoIter { let dependencies = self . dependencies . into_iter () . map (| feat | feat . as_str ()) ; std :: iter :: once (self . llvm_feature_name) . chain (dependencies) } }
    };
}

impl_555!()