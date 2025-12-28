macro_rules! deps {
    () => {
        Any!();
    };
}

macro_rules! RecurseSubmodules {
    () => {
        deps!();
        # [doc = " The `fetch.recurseSubmodules` key."] # [cfg (feature = "attributes")] pub type RecurseSubmodules = keys :: Any < validate :: RecurseSubmodules > ;
    };
}

RecurseSubmodules!()