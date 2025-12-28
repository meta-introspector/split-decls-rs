macro_rules! GenericParamCount {
    () => {
        # [derive (Default)] pub struct GenericParamCount { pub lifetimes : usize , pub types : usize , pub consts : usize , pub infer : usize , }
    };
}

GenericParamCount!()