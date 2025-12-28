macro_rules! deps {
    () => {
        CommitAutoRollback!();
        Error!();
        Repository!();
    };
}

macro_rules! impl_569 {
    () => {
        deps!();
        # [doc = " Utilities"] impl < 'repo > CommitAutoRollback < 'repo > { # [doc = " Rollback the changes previously applied and all values before the change."] pub fn rollback (mut self) -> Result < & 'repo mut crate :: Repository , crate :: config :: Error > { let repo = self . repo . take () . expect ("still present, consumed only once") ; self . rollback_inner (repo) } pub (crate) fn rollback_inner (& mut self , repo : & 'repo mut crate :: Repository ,) -> Result < & 'repo mut crate :: Repository , crate :: config :: Error > { repo . reread_values_and_clear_caches_replacing_config (OwnShared :: clone (& self . prev_config)) ? ; Ok (repo) } }
    };
}

impl_569!();