macro_rules! deps {
    () => {
        Function!();
        Type!();
    };
}

macro_rules! UnresolvedMethodCall {
    () => {
        deps!();
        # [derive (Debug)] pub struct UnresolvedMethodCall < 'db > { pub expr : InFile < ExprOrPatPtr > , pub receiver : Type < 'db > , pub name : Name , pub field_with_same_name : Option < Type < 'db > > , pub assoc_func_with_same_name : Option < Function > , }
    };
}

UnresolvedMethodCall!()