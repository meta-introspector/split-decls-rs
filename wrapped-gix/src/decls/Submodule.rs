macro_rules! deps {
    () => {
        Ignore!();
        Clone!();
    };
}

macro_rules! Submodule {
    () => {
        deps!();
        # [doc = " How to obtain a submodule's status."] # [derive (Copy , Clone , Debug , Ord , PartialOrd , Eq , PartialEq , Hash)] pub enum Submodule { # [doc = " Use the `diff.submoduleIgnore` configuration to determine, or if not set,"] # [doc = " use the submodule's own ['ignore' value](crate::Submodule::ignore) to determine"] # [doc = " which submodules participate in the status query, and to which extent."] AsConfigured { # [doc = " If `true`, default `false`, the computation will stop once the first in a ladder operations"] # [doc = " ordered from cheap to expensive shows that the submodule is dirty."] # [doc = " Thus, submodules that are clean will still impose the complete set of computation, as configured."] check_dirty : bool , } , # [doc = " Instead of the configuration, use the given ['ignore' value](crate::submodule::config::Ignore)."] # [doc = " This makes it possible to fine-tune the amount of work invested in this status, while allowing"] # [doc = " to turn off all submodule status information."] Given { # [doc = " The portion of the submodule status to ignore."] ignore : crate :: submodule :: config :: Ignore , # [doc = " If `true`, default `false`, the computation will stop once the first in a ladder operations"] # [doc = " ordered from cheap to expensive shows that the submodule is dirty."] # [doc = " Thus, submodules that are clean will still impose the complete set of computation, as given."] check_dirty : bool , } , }
    };
}

Submodule!()