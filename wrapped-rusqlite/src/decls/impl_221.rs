macro_rules! deps {
    () => {
        Row!();
        Statement!();
        Error!();
        Result!();
        Rows!();
    };
}

macro_rules! impl_221 {
    () => {
        deps!();
        # [doc = " `FallibleStreamingIterator` differs from the standard library's `Iterator`"] # [doc = " in two ways:"] # [doc = " * each call to `next` (`sqlite3_step`) can fail."] # [doc = " * returned `Row` is valid until `next` is called again or `Statement` is"] # [doc = "   reset or finalized."] # [doc = ""] # [doc = " While these iterators cannot be used with Rust `for` loops, `while let`"] # [doc = " loops offer a similar level of ergonomics:"] # [doc = " ```rust,no_run"] # [doc = " # use rusqlite::{Result, Statement};"] # [doc = " fn query(stmt: &mut Statement) -> Result<()> {"] # [doc = "     let mut rows = stmt.query([])?;"] # [doc = "     while let Some(row) = rows.next()? {"] # [doc = "         // scan columns value"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] impl < 'stmt > FallibleStreamingIterator for Rows < 'stmt > { type Error = Error ; type Item = Row < 'stmt > ; # [inline] fn advance (& mut self) -> Result < () > { if let Some (stmt) = self . stmt { match stmt . step () { Ok (true) => { self . row = Some (Row { stmt }) ; Ok (()) } Ok (false) => { let r = self . reset () ; self . row = None ; r } Err (e) => { let _ = self . reset () ; self . row = None ; Err (e) } } } else { self . row = None ; Ok (()) } } # [inline] fn get (& self) -> Option < & Row < 'stmt > > { self . row . as_ref () } }
    };
}

impl_221!();