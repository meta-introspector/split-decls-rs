macro_rules! deps {
    () => {
        Updates!();
        Inserts!();
        Result!();
        ValueRef!();
        CreateVTab!();
    };
}

macro_rules! UpdateVTab {
    () => {
        deps!();
        # [doc = " Writable virtual table instance trait."] # [doc = ""] # [doc = " (See [SQLite doc](https://sqlite.org/vtab.html#xupdate))"] pub trait UpdateVTab < 'vtab > : CreateVTab < 'vtab > { # [doc = " Delete rowid or PK"] fn delete (& mut self , arg : ValueRef < '_ >) -> Result < () > ; # [doc = " Insert: `args[0] == NULL: old rowid or PK, args[1]: new rowid or PK,"] # [doc = " args[2]: ...`"] # [doc = ""] # [doc = " Return the new rowid."] fn insert (& mut self , args : & Inserts < '_ >) -> Result < i64 > ; # [doc = " Update: `args[0] != NULL: old rowid or PK, args[1]: new row id or PK,"] # [doc = " args[2]: ...`"] fn update (& mut self , args : & Updates < '_ >) -> Result < () > ; }
    };
}

UpdateVTab!()