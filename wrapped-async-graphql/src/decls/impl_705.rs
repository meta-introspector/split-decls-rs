macro_rules! deps {
    () => {
        NodesFieldSwitcherSealed!();
        ConnectionNameType!();
        ObjectType!();
        CursorType!();
        TypeName!();
        Connection!();
        OutputType!();
        EdgeNameType!();
    };
}

macro_rules! impl_705 {
    () => {
        deps!();
        impl < Cursor , Node , ConnectionFields , EdgeFields , Name , EdgeName , NodesField > TypeName for Connection < Cursor , Node , ConnectionFields , EdgeFields , Name , EdgeName , NodesField > where Cursor : CursorType + Send + Sync , Node : OutputType , ConnectionFields : ObjectType , EdgeFields : ObjectType , Name : ConnectionNameType , EdgeName : EdgeNameType , NodesField : NodesFieldSwitcherSealed , { # [inline] fn type_name () -> Cow < 'static , str > { Name :: type_name :: < Node > () . into () } }
    };
}

impl_705!()