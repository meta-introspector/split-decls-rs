macro_rules! macro_129 {
    () => {
        # [cfg (feature = "signal")] libc_enum ! { # [doc = " Specifies how certain functions should manipulate a signal mask"] # [repr (i32)] # [non_exhaustive] # [cfg_attr (docsrs , doc (cfg (feature = "signal")))] pub enum SigmaskHow { # [doc = " The new mask is the union of the current mask and the specified set."] SIG_BLOCK , # [doc = " The new mask is the intersection of the current mask and the"] # [doc = " complement of the specified set."] SIG_UNBLOCK , # [doc = " The current mask is replaced by the specified set."] SIG_SETMASK , } }
    };
}

macro_129!()