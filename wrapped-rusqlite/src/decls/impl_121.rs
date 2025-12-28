macro_rules! deps {
    () => {
        Connection!();
        PreUpdateCase!();
        Action!();
    };
}

macro_rules! impl_121 {
    () => {
        deps!();
        impl Connection { # [doc = " Register a callback function to be invoked before"] # [doc = " a row is updated, inserted or deleted."] # [doc = ""] # [doc = " The callback parameters are:"] # [doc = ""] # [doc = " - the name of the database (\"main\", \"temp\", ...),"] # [doc = " - the name of the table that is updated,"] # [doc = " - a variant of the PreUpdateCase enum which allows access to extra functions depending"] # [doc = "   on whether it's an update, delete or insert."] # [inline] pub fn preupdate_hook < F > (& self , hook : Option < F >) where F : FnMut (Action , & str , & str , & PreUpdateCase) + Send + 'static , { self . db . borrow_mut () . preupdate_hook (hook) ; } }
    };
}

impl_121!();