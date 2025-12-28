macro_rules! deps {
    () => {
        Result!();
        VTabKind!();
        VTabConnection!();
        VTab!();
    };
}

macro_rules! CreateVTab {
    () => {
        deps!();
        # [doc = " Read-only virtual table instance trait."] # [doc = ""] # [doc = " (See [SQLite doc](https://sqlite.org/c3ref/vtab.html))"] pub trait CreateVTab < 'vtab > : VTab < 'vtab > { # [doc = " For [`EponymousOnly`](VTabKind::EponymousOnly),"] # [doc = " [`create`](CreateVTab::create) and [`destroy`](CreateVTab::destroy) are"] # [doc = " not called"] const KIND : VTabKind ; # [doc = " Create a new instance of a virtual table in response to a CREATE VIRTUAL"] # [doc = " TABLE statement. The `db` parameter is a pointer to the SQLite"] # [doc = " database connection that is executing the CREATE VIRTUAL TABLE"] # [doc = " statement."] # [doc = ""] # [doc = " Call [`connect`](VTab::connect) by default."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xcreate_method))"] fn create (db : & mut VTabConnection , aux : Option < & Self :: Aux > , args : & [& [u8]] ,) -> Result < (String , Self) > { Self :: connect (db , aux , args) } # [doc = " Destroy the underlying table implementation. This method undoes the work"] # [doc = " of [`create`](CreateVTab::create)."] # [doc = ""] # [doc = " Do nothing by default."] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#the_xdestroy_method))"] fn destroy (& self) -> Result < () > { Ok (()) } }
    };
}

CreateVTab!();