macro_rules! deps {
    () => {
        ChangeId!();
    };
}

macro_rules! Relation {
    () => {
        deps!();
        # [doc = " Identifies a relationship between this instance and another one."] # [derive (Debug , Copy , Clone , PartialOrd , PartialEq , Ord , Eq , Hash)] pub enum Relation { # [doc = " This is a parent with the given ID, which will always have at least one child"] # [doc = " assuming that empty directories are not allowed in valid trees."] # [doc = " It's also always a tree which is the start of a recursive deletion or addition."] # [doc = ""] # [doc = " The change with this relation is always emitted first."] Parent (ChangeId) , # [doc = " This is a direct or indirect child, tree or not tree, of the parent with the given ID."] ChildOfParent (ChangeId) , }
    };
}

Relation!();