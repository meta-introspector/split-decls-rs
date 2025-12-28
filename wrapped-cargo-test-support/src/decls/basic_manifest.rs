macro_rules! basic_manifest {
    () => {
        # [doc = " Generate a basic `Cargo.toml`"] pub fn basic_manifest (name : & str , version : & str) -> String { format ! (r#"
        [package]
        name = "{}"
        version = "{}"
        authors = []
        edition = "2015"
    "# , name , version) }
    };
}

basic_manifest!()