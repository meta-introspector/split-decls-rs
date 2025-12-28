macro_rules! deps {
    () => {
        Connection!();
        Result!();
    };
}

macro_rules! prepare_and_bind {
    () => {
        deps!();
        # [doc = " Captured identifiers in SQL"] # [doc = ""] # [doc = " * only SQLite `$x` / `@x` / `:x` syntax works (Rust `&x` syntax does not"] # [doc = "   work)."] # [doc = " * `$x.y` expression does not work."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust, no_run"] # [doc = " # use rusqlite::{prepare_and_bind, Connection, Result, Statement};"] # [doc = ""] # [doc = " fn misc(db: &Connection) -> Result<Statement> {"] # [doc = "     let name = \"Lisa\";"] # [doc = "     let age = 8;"] # [doc = "     let smart = true;"] # [doc = "     Ok(prepare_and_bind!(db, \"SELECT $name, @age, :smart;\"))"] # [doc = " }"] # [doc = " ```"] # [cfg (feature = "rusqlite-macros")] # [macro_export] macro_rules ! prepare_and_bind { ($ conn : expr , $ sql : literal) => { { let mut stmt = $ conn . prepare ($ sql) ?; $ crate :: __bind ! (stmt $ sql) ; stmt } } ; }
    };
}

prepare_and_bind!()