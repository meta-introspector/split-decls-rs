macro_rules! as_name_opt {
    () => {
        fn as_name_opt (name : Option < impl AsName >) -> Name { name . map_or_else (Name :: missing , | name | name . as_name ()) }
    };
}

as_name_opt!();