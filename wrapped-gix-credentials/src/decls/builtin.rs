macro_rules! deps {
    () => {
        Result!();
        Kind!();
        Program!();
        Action!();
    };
}

macro_rules! builtin {
    () => {
        deps!();
        # [doc = " Call the `git credential` helper program performing the given `action`, which reads all context from the git configuration"] # [doc = " and does everything `git` typically does. The `action` should have been created with [`helper::Action::get_for_url()`] to"] # [doc = " contain only the URL to kick off the process, or should be created by [`helper::NextAction`]."] # [doc = ""] # [doc = " If more control is required, use the [`Cascade`][helper::Cascade] type."] # [allow (clippy :: result_large_err)] pub fn builtin (action : helper :: Action) -> protocol :: Result { protocol :: helper_outcome_to_result (helper :: invoke (& mut Program :: from_kind (program :: Kind :: Builtin) , & action) ? , action ,) }
    };
}

builtin!();