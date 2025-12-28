macro_rules! Store {
    () => {
        # [doc = " The git reference store."] # [doc = " TODO: Figure out if handles are needed at all, which depends on the ref-table implementation."] # [allow (dead_code)] pub (crate) struct Store { inner : store :: State , }
    };
}

Store!()