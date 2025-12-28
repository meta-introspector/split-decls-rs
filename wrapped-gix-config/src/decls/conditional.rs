macro_rules! deps {
    () => {
        Options!();
    };
}

macro_rules! conditional {
    () => {
        deps!();
        # [doc = ""] pub mod conditional { # [doc = " Options to handle conditional includes like `includeIf.<condition>.path`."] # [derive (Clone , Copy , Default)] pub struct Context < 'a > { # [doc = " The location of the .git directory. If `None`, `gitdir` conditions cause an error."] # [doc = ""] # [doc = " Used for conditional includes, e.g. `includeIf.gitdir:…` or `includeIf:gitdir/i…`."] pub git_dir : Option < & 'a std :: path :: Path > , # [doc = " The name of the branch that is currently checked out. If `None`, `onbranch` conditions cause an error."] # [doc = ""] # [doc = " Used for conditional includes, e.g. `includeIf.onbranch:main.…`"] pub branch_name : Option < & 'a gix_ref :: FullNameRef > , } }
    };
}

conditional!()