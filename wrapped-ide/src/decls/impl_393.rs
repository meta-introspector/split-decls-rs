macro_rules! deps {
    () => {
        TestAttr!();
    };
}

macro_rules! impl_393 {
    () => {
        deps!();
        impl TestAttr { fn from_fn (db : & dyn HirDatabase , fn_def : hir :: Function) -> TestAttr { TestAttr { ignore : fn_def . is_ignore (db) } } }
    };
}

impl_393!();