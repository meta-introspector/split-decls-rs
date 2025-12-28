macro_rules! deps {
    () => {
        Result!();
    };
}

macro_rules! GhExecutor {
    () => {
        deps!();
        pub trait GhExecutor { fn repo_fork (& self , repo_url : & str , target_org : & str) -> Result < () > ; fn repo_view (& self , forked_repo_url : & str) -> Result < bool > ; }
    };
}

GhExecutor!();