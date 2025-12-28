macro_rules! deps {
    () => {
        SubmoduleInfo!();
        Result!();
    };
}

macro_rules! GitRepositoryOperations {
    () => {
        deps!();
        pub trait GitRepositoryOperations { fn git_add_all (& self , repo_path : & Path) -> Result < Output > ; fn git_commit (& self , repo_path : & Path , message : & str) -> Result < Output > ; fn git_push (& self , repo_path : & Path , remote : & str , branch : & str) -> Result < Output > ; fn git_status (& self , repo_path : & Path) -> Result < Output > ; fn git_submodule_add (& self , repo_path : & Path , url : & str , path : & str , name : Option < & str > , branch : Option < & str > ,) -> Result < Output > ; fn git_submodule_update (& self , repo_path : & Path , init : bool , recursive : bool) -> Result < Output > ; fn git_submodule_status (& self , repo_path : & Path) -> Result < Output > ; fn git_submodule_remove (& self , repo_path : & Path , path : & str) -> Result < Output > ; fn submodules (& self , repo_path : & Path) -> Result < Vec < SubmoduleInfo > > ; }
    };
}

GitRepositoryOperations!();