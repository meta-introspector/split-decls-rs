macro_rules! deps {
    () => {
        Program!();
    };
}

macro_rules! Command {
    () => {
        deps!();
        # [derive (Clone)] pub (crate) struct Command { program : Program , args : Vec < OsString > , env : Vec < (OsString , OsString) > , env_remove : Vec < OsString > , env_clear : bool , }
    };
}

Command!()