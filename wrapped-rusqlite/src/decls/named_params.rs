macro_rules! deps {
    () => {
        Connection!();
        Result!();
    };
}

macro_rules! named_params {
    () => {
        deps!();
        # [doc = " A macro making it more convenient to pass lists of named parameters"] # [doc = " as a `&[(&str, &dyn ToSql)]`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Result, Connection, named_params};"] # [doc = ""] # [doc = " struct Person {"] # [doc = "     name: String,"] # [doc = "     age_in_years: u8,"] # [doc = "     data: Option<Vec<u8>>,"] # [doc = " }"] # [doc = ""] # [doc = " fn add_person(conn: &Connection, person: &Person) -> Result<()> {"] # [doc = "     conn.execute("] # [doc = "         \"INSERT INTO person (name, age_in_years, data)"] # [doc = "          VALUES (:name, :age, :data)\","] # [doc = "         named_params! {"] # [doc = "             \":name\": person.name,"] # [doc = "             \":age\": person.age_in_years,"] # [doc = "             \":data\": person.data,"] # [doc = "         },"] # [doc = "     )?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [macro_export] macro_rules ! named_params { () => { & [] as & [(& str , & dyn $ crate :: ToSql)] } ; ($ ($ param_name : literal : $ param_val : expr) ,+ $ (,) ?) => { & [$ (($ param_name , &$ param_val as & dyn $ crate :: ToSql)) ,+] as & [(& str , & dyn $ crate :: ToSql)] } ; }
    };
}

named_params!()