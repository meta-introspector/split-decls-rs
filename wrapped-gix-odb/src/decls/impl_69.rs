macro_rules! deps {
    () => {
        Store!();
        Handle!();
    };
}

macro_rules! impl_69 {
    () => {
        deps!();
        impl super :: Handle < Rc < super :: Store > > { # [doc = " Convert a ref counted store into one that is ref-counted and thread-safe, by creating a new Store."] pub fn into_arc (self) -> std :: io :: Result < super :: Handle < Arc < super :: Store > > > { let store = Arc :: new (super :: Store :: try_from (self . store_ref ()) ?) ; let mut cache = store . to_handle_arc () ; cache . refresh = self . refresh ; cache . max_recursion_depth = self . max_recursion_depth ; Ok (cache) } }
    };
}

impl_69!();