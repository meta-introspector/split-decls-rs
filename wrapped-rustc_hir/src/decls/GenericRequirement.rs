macro_rules! GenericRequirement {
    () => {
        # [doc = " The requirement imposed on the generics of a lang item"] pub enum GenericRequirement { # [doc = " No restriction on the generics"] None , # [doc = " A minimum number of generics that is demanded on a lang item"] Minimum (usize) , # [doc = " The number of generics must match precisely as stipulated"] Exact (usize) , }
    };
}

GenericRequirement!()