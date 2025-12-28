macro_rules! deps {
    () => {
        Statement!();
        AndThenRows!();
        MappedRows!();
        Rows!();
        Row!();
        Result!();
        Map!();
    };
}

macro_rules! impl_212 {
    () => {
        deps!();
        impl < 'stmt > Rows < 'stmt > { # [inline] fn reset (& mut self) -> Result < () > { if let Some (stmt) = self . stmt . take () { stmt . reset () } else { Ok (()) } } # [doc = " Attempt to get the next row from the query. Returns `Ok(Some(Row))` if"] # [doc = " there is another row, `Err(...)` if there was an error"] # [doc = " getting the next row, and `Ok(None)` if all rows have been retrieved."] # [doc = ""] # [doc = " ## Note"] # [doc = ""] # [doc = " This interface is not compatible with Rust's `Iterator` trait, because"] # [doc = " the lifetime of the returned row is tied to the lifetime of `self`."] # [doc = " This is a fallible \"streaming iterator\". For a more natural interface,"] # [doc = " consider using [`query_map`](Statement::query_map) or"] # [doc = " [`query_and_then`](Statement::query_and_then) instead, which"] # [doc = " return types that implement `Iterator`."] # [expect (clippy :: should_implement_trait)] # [inline] pub fn next (& mut self) -> Result < Option < & Row < 'stmt > > > { self . advance () ? ; Ok ((* self) . get ()) } # [doc = " Map over this `Rows`, converting it to a [`Map`], which"] # [doc = " implements `FallibleIterator`."] # [doc = " ```rust,no_run"] # [doc = " use fallible_iterator::FallibleIterator;"] # [doc = " # use rusqlite::{Result, Statement};"] # [doc = " fn query(stmt: &mut Statement) -> Result<Vec<i64>> {"] # [doc = "     let rows = stmt.query([])?;"] # [doc = "     rows.map(|r| r.get(0)).collect()"] # [doc = " }"] # [doc = " ```"] # [inline] pub fn map < F , B > (self , f : F) -> Map < 'stmt , F > where F : FnMut (& Row < '_ >) -> Result < B > , { Map { rows : self , f } } # [doc = " Map over this `Rows`, converting it to a [`MappedRows`], which"] # [doc = " implements `Iterator`."] # [inline] pub fn mapped < F , B > (self , f : F) -> MappedRows < 'stmt , F > where F : FnMut (& Row < '_ >) -> Result < B > , { MappedRows { rows : self , map : f } } # [doc = " Map over this `Rows` with a fallible function, converting it to a"] # [doc = " [`AndThenRows`], which implements `Iterator` (instead of"] # [doc = " `FallibleStreamingIterator`)."] # [inline] pub fn and_then < F , T , E > (self , f : F) -> AndThenRows < 'stmt , F > where F : FnMut (& Row < '_ >) -> Result < T , E > , { AndThenRows { rows : self , map : f } } # [doc = " Give access to the underlying statement"] # [must_use] pub fn as_ref (& self) -> Option < & Statement < 'stmt > > { self . stmt } }
    };
}

impl_212!()