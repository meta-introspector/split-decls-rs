macro_rules! Env {
    () => {
        # [derive (Debug , Clone)] # [non_exhaustive] pub enum Env { Owned (OsString) , Arced (Arc < OsStr >) , }
    };
}

Env!()