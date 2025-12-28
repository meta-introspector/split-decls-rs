macro_rules! deps {
    () => {
        OutputType!();
        EdgeNameType!();
        ObjectType!();
        Connection!();
        EmptyFields!();
        NodesFieldSwitcherSealed!();
        ConnectionNameType!();
        CursorType!();
    };
}

macro_rules! impl_701 {
    () => {
        deps!();
        impl < Cursor , Node , NodesField , EdgeFields , Name , EdgeName > Connection < Cursor , Node , EmptyFields , EdgeFields , Name , EdgeName , NodesField > where Cursor : CursorType + Send + Sync , Node : OutputType , EdgeFields : ObjectType , Name : ConnectionNameType , EdgeName : EdgeNameType , NodesField : NodesFieldSwitcherSealed , { # [doc = " Create a new connection."] # [inline] pub fn new (has_previous_page : bool , has_next_page : bool) -> Self { Connection { _mark1 : PhantomData , _mark2 : PhantomData , _mark3 : PhantomData , additional_fields : EmptyFields , has_previous_page , has_next_page , edges : Vec :: new () , } } }
    };
}

impl_701!();