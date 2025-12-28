macro_rules! deps {
    () => {
        Edge!();
        EdgeNameType!();
        ObjectType!();
        OutputType!();
        ComplexObject!();
        CursorType!();
    };
}

macro_rules! impl_724 {
    () => {
        deps!();
        # [ComplexObject (internal)] impl < Cursor , Node , EdgeFields , Name > Edge < Cursor , Node , EdgeFields , Name > where Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , Name : EdgeNameType , { # [doc = " A cursor for use in pagination"] async fn cursor (& self) -> String { self . cursor . encode_cursor () } }
    };
}

impl_724!();