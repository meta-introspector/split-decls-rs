macro_rules! deps {
    () => {
        Platform!();
        Path!();
        AttributeStack!();
    };
}

macro_rules! impl_40 {
    () => {
        deps!();
        # [doc = " Platform retrieval"] impl AttributeStack < '_ > { # [doc = " Append the `relative` path to the root directory of the cache and load all attribute or ignore files on the way as needed."] # [doc = " Use `mode` to specify what kind of item lives at `relative` - directories may match against rules specifically."] # [doc = " If `mode` is `None`, the item at `relative` is assumed to be a file."] # [doc = ""] # [doc = " The returned platform may be used to access the actual attribute or ignore information."] # [doc (alias = "is_path_ignored" , alias = "git2")] pub fn at_path (& mut self , relative : impl AsRef < std :: path :: Path > , mode : Option < gix_index :: entry :: Mode > ,) -> std :: io :: Result < gix_worktree :: stack :: Platform < '_ > > { self . inner . at_path (relative . as_ref () , mode , & self . repo . objects) } # [doc = " Obtain a platform for attribute or ignore lookups from a repo-`relative` path, typically obtained from an index entry."] # [doc = " `mode` should reflect whether it's a directory or not, or left at `None` if unknown."] # [doc = ""] # [doc = " If `relative` ends with `/` and `mode` is `None`, it is automatically assumed to be a directory."] pub fn at_entry (& mut self , relative : impl ToNormalPathComponents , mode : Option < gix_index :: entry :: Mode > ,) -> std :: io :: Result < gix_worktree :: stack :: Platform < '_ > > { self . inner . at_path (relative , mode , & self . repo . objects) } }
    };
}

impl_40!();