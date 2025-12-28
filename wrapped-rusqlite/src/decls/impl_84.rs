macro_rules! deps {
    () => {
        DbConfig!();
        Result!();
        Connection!();
    };
}

macro_rules! impl_84 {
    () => {
        deps!();
        impl Connection { # [doc = " Returns the current value of a `config`."] # [doc = ""] # [doc = " - `SQLITE_DBCONFIG_ENABLE_FKEY`: return `false` or `true` to indicate"] # [doc = "   whether FK enforcement is off or on"] # [doc = " - `SQLITE_DBCONFIG_ENABLE_TRIGGER`: return `false` or `true` to indicate"] # [doc = "   whether triggers are disabled or enabled"] # [doc = " - `SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER`: return `false` or `true` to"] # [doc = "   indicate whether `fts3_tokenizer` are disabled or enabled"] # [doc = " - `SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE`: return `false` to indicate"] # [doc = "   checkpoints-on-close are not disabled or `true` if they are"] # [doc = " - `SQLITE_DBCONFIG_ENABLE_QPSG`: return `false` or `true` to indicate"] # [doc = "   whether the QPSG is disabled or enabled"] # [doc = " - `SQLITE_DBCONFIG_TRIGGER_EQP`: return `false` to indicate"] # [doc = "   output-for-trigger are not disabled or `true` if it is"] # [inline] pub fn db_config (& self , config : DbConfig) -> Result < bool > { let c = self . db . borrow () ; unsafe { let mut val = 0 ; check (ffi :: sqlite3_db_config (c . db () , config as c_int , - 1 , & mut val ,)) ? ; Ok (val != 0) } } # [doc = " Make configuration changes to a database connection"] # [doc = ""] # [doc = " - `SQLITE_DBCONFIG_ENABLE_FKEY`: `false` to disable FK enforcement,"] # [doc = "   `true` to enable FK enforcement"] # [doc = " - `SQLITE_DBCONFIG_ENABLE_TRIGGER`: `false` to disable triggers, `true`"] # [doc = "   to enable triggers"] # [doc = " - `SQLITE_DBCONFIG_ENABLE_FTS3_TOKENIZER`: `false` to disable"] # [doc = "   `fts3_tokenizer()`, `true` to enable `fts3_tokenizer()`"] # [doc = " - `SQLITE_DBCONFIG_NO_CKPT_ON_CLOSE`: `false` (the default) to enable"] # [doc = "   checkpoints-on-close, `true` to disable them"] # [doc = " - `SQLITE_DBCONFIG_ENABLE_QPSG`: `false` to disable the QPSG, `true` to"] # [doc = "   enable QPSG"] # [doc = " - `SQLITE_DBCONFIG_TRIGGER_EQP`: `false` to disable output for trigger"] # [doc = "   programs, `true` to enable it"] # [inline] pub fn set_db_config (& self , config : DbConfig , new_val : bool) -> Result < bool > { let c = self . db . borrow_mut () ; unsafe { let mut val = 0 ; check (ffi :: sqlite3_db_config (c . db () , config as c_int , new_val as c_int , & mut val ,)) ? ; Ok (val != 0) } } }
    };
}

impl_84!()