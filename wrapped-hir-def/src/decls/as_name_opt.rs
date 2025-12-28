macro_rules! as_name_opt {
    () => {
        # [inline] fn as_name_opt (name : Option < ast :: Name >) -> Name { name . map_or_else (Name :: missing , | it | it . as_name ()) }
    };
}

as_name_opt!();