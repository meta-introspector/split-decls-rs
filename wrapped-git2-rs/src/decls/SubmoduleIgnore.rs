macro_rules! deps {
    () => {
        Submodule!();
    };
}

macro_rules! SubmoduleIgnore {
    () => {
        deps!();
        # [doc = " Submodule ignore values"] # [doc = ""] # [doc = " These values represent settings for the `submodule.$name.ignore`"] # [doc = " configuration value which says how deeply to look at the working"] # [doc = " directory when getting the submodule status."] # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum SubmoduleIgnore { # [doc = " Use the submodule's configuration"] Unspecified , # [doc = " Any change or untracked file is considered dirty"] None , # [doc = " Only dirty if tracked files have changed"] Untracked , # [doc = " Only dirty if HEAD has moved"] Dirty , # [doc = " Never dirty"] All , }
    };
}

SubmoduleIgnore!()