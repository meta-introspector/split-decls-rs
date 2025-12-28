macro_rules! Parents {
    () => {
        # [doc = " Specify how to handle commit parents during traversal."] # [derive (Default , Copy , Clone)] pub enum Parents { # [doc = " Traverse all parents, useful for traversing the entire ancestry."] # [default] All , # [doc = " Only traverse along the first parent, which commonly ignores all branches."] First , }
    };
}

Parents!();