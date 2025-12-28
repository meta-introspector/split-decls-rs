macro_rules! Sorting {
    () => {
        # [doc = " Sorting to use for the topological walk."] # [doc = ""] # [doc = " ### Sample History"] # [doc = ""] # [doc = " The following history will be referred to for explaining how the sort order works, with the number denoting the commit timestamp"] # [doc = " (*their X-alignment doesn't matter*)."] # [doc = ""] # [doc = " ```text"] # [doc = " ---1----2----4----7 <- second parent of 8"] # [doc = "     \\              \\"] # [doc = "      3----5----6----8---"] # [doc = " ```"] # [derive (Clone , Copy , Debug , Default)] pub enum Sorting { # [doc = " Show no parents before all of its children are shown, but otherwise show"] # [doc = " commits in the commit timestamp order."] # [doc = ""] # [doc = " This is equivalent to `git rev-list --date-order`."] # [default] DateOrder , # [doc = " Show no parents before all of its children are shown, and avoid"] # [doc = " showing commits on multiple lines of history intermixed."] # [doc = ""] # [doc = " In the *sample history* the order would be `8, 6, 5, 3, 7, 4, 2, 1`."] # [doc = " This is equivalent to `git rev-list --topo-order`."] TopoOrder , }
    };
}

Sorting!();