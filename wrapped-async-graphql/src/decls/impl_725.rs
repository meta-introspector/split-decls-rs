macro_rules! deps {
    () => {
        ObjectType!();
        TypeName!();
        Edge!();
        CursorType!();
        OutputType!();
        EdgeNameType!();
    };
}

macro_rules! impl_725 {
    () => {
        deps!();
        impl < Cursor , Node , EdgeFields , Name > TypeName for Edge < Cursor , Node , EdgeFields , Name > where Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , Name : EdgeNameType , { # [inline] fn type_name () -> Cow < 'static , str > { Name :: type_name :: < Node > () . into () } }
    };
}

impl_725!()