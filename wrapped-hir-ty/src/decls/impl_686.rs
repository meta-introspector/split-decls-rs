macro_rules! deps {
    () => {
        CandidateId!();
        HirDatabase!();
    };
}

macro_rules! impl_686 {
    () => {
        deps!();
        impl CandidateId { fn container (self , db : & dyn HirDatabase) -> ItemContainerId { match self { CandidateId :: FunctionId (id) => id . loc (db) . container , CandidateId :: ConstId (id) => id . loc (db) . container , } } }
    };
}

impl_686!();