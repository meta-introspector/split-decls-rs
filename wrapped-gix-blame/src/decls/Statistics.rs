macro_rules! Statistics {
    () => {
        # [doc = " Additional information about the performed operations."] # [derive (Debug , Default , Copy , Clone)] pub struct Statistics { # [doc = " The amount of commits it traversed until the blame was complete."] pub commits_traversed : usize , # [doc = " The amount of trees that were decoded to find the entry of the file to blame."] pub trees_decoded : usize , # [doc = " The amount of tree-diffs to see if the filepath was added, deleted or modified. These diffs"] # [doc = " are likely partial as they are cancelled as soon as a change to the blamed file is"] # [doc = " detected."] pub trees_diffed : usize , # [doc = " The amount of tree-diffs to see if the file was moved (or rewritten, in git terminology)."] # [doc = " These diffs are likely partial as they are cancelled as soon as a change to the blamed file"] # [doc = " is detected."] pub trees_diffed_with_rewrites : usize , # [doc = " The amount of blobs there were compared to each other to learn what changed between commits."] # [doc = " Note that in order to diff a blob, one needs to load both versions from the database."] pub blobs_diffed : usize , }
    };
}

Statistics!();