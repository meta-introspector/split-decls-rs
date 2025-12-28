macro_rules! CanAccessMutGlobal {
    () => {
        # [derive (Copy , Clone , PartialEq)] pub (crate) enum CanAccessMutGlobal { No , Yes , }
    };
}

CanAccessMutGlobal!()