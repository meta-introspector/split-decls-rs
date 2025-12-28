macro_rules! Item {
    () => {
        # [doc = " An item to match, input to various matching operations."] # [derive (Debug , Copy , Clone)] pub struct Item < 'a > { # [doc = " The full name of the references, like `refs/heads/main`"] pub full_ref_name : & 'a BStr , # [doc = " The id that `full_ref_name` points to, which typically is a commit, but can also be a tag object (or anything else)."] pub target : & 'a oid , # [doc = " The object an annotated tag is pointing to, if `target` is an annotated tag."] pub object : Option < & 'a oid > , }
    };
}

Item!();