macro_rules! UnresolvedError {
    () => {
        # [doc = " Error validating names in Cargo."] # [derive (Debug , thiserror :: Error)] # [error ("manifest field was not resolved")] # [non_exhaustive] # [cfg_attr (feature = "unstable-schema" , derive (schemars :: JsonSchema))] pub struct UnresolvedError ;
    };
}

UnresolvedError!();