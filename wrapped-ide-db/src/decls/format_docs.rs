macro_rules! deps {
    () => {
        Documentation!();
    };
}

macro_rules! format_docs {
    () => {
        deps!();
        pub fn format_docs (src : & Documentation) -> String { format_docs_ (src . as_str ()) }
    };
}

format_docs!()