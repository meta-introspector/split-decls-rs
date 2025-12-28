macro_rules! deps {
    () => {
        GitDetails!();
        GitInfoTrait!();
    };
}

macro_rules! impl_17 {
    () => {
        deps!();
        impl GitInfoTrait for GitDetails { fn git_repo (& self) -> Option < & str > { match self { GitDetails :: Info (info) => Some (& info . repo_url) , _ => None , } } fn git_path (& self) -> Option < & str > { None } fn our_fork_github (& self) -> Option < & str > { None } fn our_branch (& self) -> Option < & str > { match self { GitDetails :: Info (info) => Some (& info . branch) , _ => None , } } }
    };
}

impl_17!()