macro_rules! GenericKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq , Hash , TypeFoldable , TypeVisitable)] pub enum GenericKind < 'tcx > { Param (ty :: ParamTy) , Placeholder (ty :: PlaceholderType) , Alias (ty :: AliasTy < 'tcx >) , }
    };
}

GenericKind!();