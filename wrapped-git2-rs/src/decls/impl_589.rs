macro_rules! deps {
    () => {
        Rebase!();
        Error!();
        RebaseOperation!();
    };
}

macro_rules! impl_589 {
    () => {
        deps!();
        impl < 'rebase > Iterator for Rebase < 'rebase > { type Item = Result < RebaseOperation < 'rebase > , Error > ; # [doc = " Performs the next rebase operation and returns the information about it."] # [doc = " If the operation is one that applies a patch (which is any operation except"] # [doc = " GitRebaseOperation::Exec) then the patch will be applied and the index and"] # [doc = " working directory will be updated with the changes.  If there are conflicts,"] # [doc = " you will need to address those before committing the changes."] fn next (& mut self) -> Option < Result < RebaseOperation < 'rebase > , Error > > { let mut out = ptr :: null_mut () ; unsafe { try_call_iter ! (raw :: git_rebase_next (& mut out , self . raw)) ; Some (Ok (RebaseOperation :: from_raw (out))) } } }
    };
}

impl_589!()