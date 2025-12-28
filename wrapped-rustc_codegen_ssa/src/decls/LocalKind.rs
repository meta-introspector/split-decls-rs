macro_rules! LocalKind {
    () => {
        # [derive (Copy , Clone , PartialEq , Eq)] enum LocalKind { ZST , # [doc = " A local that requires an alloca."] Memory , # [doc = " A scalar or a scalar pair local that is neither defined nor used."] Unused , # [doc = " A scalar or a scalar pair local with a single definition that dominates all uses."] SSA (DefLocation) , }
    };
}

LocalKind!()