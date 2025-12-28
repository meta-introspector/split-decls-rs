macro_rules! GenericRequirement {
    () => {
        pub enum GenericRequirement { None , Minimum (usize) , Exact (usize) , }
    };
}

GenericRequirement!()