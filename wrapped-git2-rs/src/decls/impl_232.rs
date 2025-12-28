macro_rules! deps {
    () => {
        Branch!();
        Remote!();
        Binding!();
        Branches!();
        BranchType!();
        Error!();
    };
}

macro_rules! impl_232 {
    () => {
        deps!();
        impl < 'repo > Iterator for Branches < 'repo > { type Item = Result < (Branch < 'repo > , BranchType) , Error > ; fn next (& mut self) -> Option < Result < (Branch < 'repo > , BranchType) , Error > > { let mut ret = ptr :: null_mut () ; let mut typ = raw :: GIT_BRANCH_LOCAL ; unsafe { try_call_iter ! (raw :: git_branch_next (& mut ret , & mut typ , self . raw)) ; let typ = match typ { raw :: GIT_BRANCH_LOCAL => BranchType :: Local , raw :: GIT_BRANCH_REMOTE => BranchType :: Remote , n => panic ! ("unexected branch type: {}" , n) , } ; Some (Ok ((Branch :: wrap (Binding :: from_raw (ret)) , typ))) } } }
    };
}

impl_232!()