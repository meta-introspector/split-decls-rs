macro_rules! DEFAULT_BRANCH_NAME {
    () => {
        # [doc = " The name of the branch to use if non is configured via git configuration."] # [doc = ""] # [doc = " # Deviation"] # [doc = ""] # [doc = " We use `main` instead of `master`."] pub const DEFAULT_BRANCH_NAME : & str = "main" ;
    };
}

DEFAULT_BRANCH_NAME!()