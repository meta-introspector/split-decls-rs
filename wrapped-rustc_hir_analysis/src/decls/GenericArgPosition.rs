macro_rules! GenericArgPosition {
    () => {
        # [doc = " Denotes the \"position\" of a generic argument, indicating if it is a generic type,"] # [doc = " generic function or generic method call."] # [derive (Copy , Clone , PartialEq)] pub (crate) enum GenericArgPosition { Type , Value , MethodCall , }
    };
}

GenericArgPosition!()