macro_rules! GenericParamList {
    () => {
        # [derive (Default)] pub struct GenericParamList (pub Vec < GenericParam >) ;
    };
}

GenericParamList!()