macro_rules! Needs {
    () => {
        # [derive (Copy , Clone , Debug , PartialEq , Eq)] pub enum Needs { MutPlace , None , }
    };
}

Needs!()