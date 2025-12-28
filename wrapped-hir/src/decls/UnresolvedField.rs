macro_rules! deps {
    () => {
        Type!();
    };
}

macro_rules! UnresolvedField {
    () => {
        deps!();
        # [derive (Debug)] pub struct UnresolvedField < 'db > { pub expr : InFile < ExprOrPatPtr > , pub receiver : Type < 'db > , pub name : Name , pub method_with_same_name_exists : bool , }
    };
}

UnresolvedField!();