macro_rules! deps {
    () => {
        Editor!();
    };
}

macro_rules! Cursor {
    () => {
        deps!();
        # [doc = " A way to constrain all [tree-edits](Editor) to a given subtree."] pub struct Cursor < 'a , 'find > { # [doc = " The underlying editor"] parent : & 'a mut Editor < 'find > , # [doc = " Our own location, used as prefix for all operations."] # [doc = " Note that it's assumed to always contain a tree."] prefix : BString , }
    };
}

Cursor!();