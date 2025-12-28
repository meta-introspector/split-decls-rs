macro_rules! deps {
    () => {
        Repository!();
        IsActiveState!();
        ModulesSnapshot!();
        IndexPersistedOrInMemory!();
    };
}

macro_rules! SharedState {
    () => {
        deps!();
        # [doc = " A platform maintaining state needed to interact with submodules, created by [`Repository::submodules()]."] pub (crate) struct SharedState < 'repo > { pub repo : & 'repo Repository , pub (crate) modules : ModulesSnapshot , is_active : RefCell < Option < IsActiveState > > , index : RefCell < Option < IndexPersistedOrInMemory > > , }
    };
}

SharedState!();