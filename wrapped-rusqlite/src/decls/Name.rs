macro_rules! deps {
    () => {
        Result!();
        Named!();
    };
}

macro_rules! Name {
    () => {
        deps!();
        # [doc = " Database, table, column, collation, function, module, vfs name"] pub trait Name : std :: fmt :: Debug { # [doc = " As C string"] fn as_cstr (& self) -> Result < Named < '_ > > ; }
    };
}

Name!()