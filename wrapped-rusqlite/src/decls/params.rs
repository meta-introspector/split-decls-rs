macro_rules! deps {
    () => {
        Result!();
        Connection!();
    };
}

macro_rules! params {
    () => {
        deps!();
        # [doc = " A macro making it more convenient to pass longer lists of"] # [doc = " parameters as a `&[&dyn ToSql]`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Result, Connection, params};"] # [doc = ""] # [doc = " struct Person {"] # [doc = "     name: String,"] # [doc = "     age_in_years: u8,"] # [doc = "     data: Option<Vec<u8>>,"] # [doc = " }"] # [doc = ""] # [doc = " fn add_person(conn: &Connection, person: &Person) -> Result<()> {"] # [doc = "     conn.execute("] # [doc = "         \"INSERT INTO person(name, age_in_years, data) VALUES (?1, ?2, ?3)\","] # [doc = "         params![person.name, person.age_in_years, person.data],"] # [doc = "     )?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! params { () => { & [] as & [& dyn $ crate :: ToSql] } ; ($ ($ param : expr) ,+ $ (,) ?) => { & [$ (&$ param as & dyn $ crate :: ToSql) ,+] as & [& dyn $ crate :: ToSql] } ; }
    };
}

params!()