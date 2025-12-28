macro_rules! ResolveWith {
    () => {
        # [doc = " Decide how to resolve tree-related conflicts, but only those that have [no way of being correct](ResolutionFailure)."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] pub enum ResolveWith { # [doc = " On irreconcilable conflict, choose neither *our* nor *their* state, but keep the common *ancestor* state instead."] Ancestor , # [doc = " On irreconcilable conflict, choose *our* side."] # [doc = ""] # [doc = " Note that in order to get something equivalent to *theirs*, put *theirs* into the side of *ours*,"] # [doc = " swapping the sides essentially."] Ours , }
    };
}

ResolveWith!();