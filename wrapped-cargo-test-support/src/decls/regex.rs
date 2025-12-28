macro_rules! regex {
    () => {
        # [doc = " This makes it easier to write regex replacements that are guaranteed to only"] # [doc = " get compiled once"] macro_rules ! regex { ($ re : literal $ (,) ?) => { { static RE : std :: sync :: OnceLock < regex :: Regex > = std :: sync :: OnceLock :: new () ; RE . get_or_init (|| regex :: Regex :: new ($ re) . unwrap ()) } } ; }
    };
}

regex!();