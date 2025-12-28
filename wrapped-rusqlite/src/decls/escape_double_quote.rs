macro_rules! escape_double_quote {
    () => {
        # [doc = " Escape double-quote (`\"`) character occurrences by"] # [doc = " doubling them (`\"\"`)."] # [must_use] pub fn escape_double_quote (identifier : & str) -> Cow < '_ , str > { if identifier . contains ('"') { Owned (identifier . replace ('"' , "\"\"")) } else { Borrowed (identifier) } }
    };
}

escape_double_quote!()