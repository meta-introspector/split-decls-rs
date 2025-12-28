macro_rules! deps {
    () => {
        OutputType!();
        ObjectType!();
        EdgeNameType!();
        CursorType!();
        Edge!();
    };
}

macro_rules! impl_726 {
    () => {
        deps!();
        impl < Cursor , Node , EdgeFields , Name > Edge < Cursor , Node , EdgeFields , Name > where Name : EdgeNameType , Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , { # [doc = " Create a new edge, it can have some additional fields."] # [inline] pub fn with_additional_fields (cursor : Cursor , node : Node , additional_fields : EdgeFields ,) -> Self { Self { _mark : PhantomData , cursor , node , additional_fields , } } }
    };
}

impl_726!()