macro_rules! deps {
    () => {
        OutputType!();
        EdgeNameType!();
        CursorType!();
        Edge!();
        EmptyFields!();
    };
}

macro_rules! impl_727 {
    () => {
        deps!();
        impl < Cursor , Node , Name > Edge < Cursor , Node , EmptyFields , Name > where Cursor : CursorType + Send + Sync , Node : OutputType , Name : EdgeNameType , { # [doc = " Create a new edge."] # [inline] pub fn new (cursor : Cursor , node : Node) -> Self { Self { _mark : PhantomData , cursor , node , additional_fields : EmptyFields , } } }
    };
}

impl_727!()