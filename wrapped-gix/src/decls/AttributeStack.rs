macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! AttributeStack {
    () => {
        deps!();
        # [doc = " A utility to access `.gitattributes` and `.gitignore` information efficiently."] # [cfg (any (feature = "attributes" , feature = "excludes"))] pub struct AttributeStack < 'repo > { # [doc = " The owning repository."] pub repo : & 'repo Repository , pub (crate) inner : gix_worktree :: Stack , }
    };
}

AttributeStack!()