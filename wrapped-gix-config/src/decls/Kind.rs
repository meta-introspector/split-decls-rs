macro_rules! deps {
    () => {
        Source!();
    };
}

macro_rules! Kind {
    () => {
        deps!();
        # [doc = " The category of a [`Source`], in order of ascending precedence."] # [derive (Clone , Copy , Debug , Eq , PartialEq , Hash , Ord , PartialOrd)] pub enum Kind { # [doc = " A special configuration file that ships with the git installation, and is thus tied to the used git binary."] GitInstallation , # [doc = " A source shared for the entire system."] System , # [doc = " Application specific configuration unique for each user of the `System`."] Global , # [doc = " Configuration relevant only to the repository, possibly including the worktree."] Repository , # [doc = " Configuration specified after all other configuration was loaded for the purpose of overrides."] Override , }
    };
}

Kind!();