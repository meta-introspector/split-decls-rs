macro_rules! deps {
    () => {
        LintStore!();
    };
}

macro_rules! unerased_lint_store {
    () => {
        deps!();
        # [doc = " Extract the [`LintStore`] from [`Session`]."] # [doc = ""] # [doc = " This function exists because [`Session::lint_store`] is type-erased."] pub fn unerased_lint_store (sess : & Session) -> & LintStore { let store : & dyn Any = sess . lint_store . as_deref () . unwrap () ; store . downcast_ref () . unwrap () }
    };
}

unerased_lint_store!()