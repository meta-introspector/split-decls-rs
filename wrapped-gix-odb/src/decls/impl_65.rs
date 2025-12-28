macro_rules! deps {
    () => {
        RefreshMode!();
        Store!();
        Handle!();
        Cache!();
    };
}

macro_rules! impl_65 {
    () => {
        deps!();
        # [doc = " Handle creation"] impl super :: Store { # [doc = " The amount of times a ref-delta base can be followed when multi-indices are involved."] pub const INITIAL_MAX_RECURSION_DEPTH : usize = 32 ; # [doc = " Create a new cache filled with a handle to this store, if this store is supporting shared ownership."] # [doc = ""] # [doc = " Note that the actual type of `OwnShared` depends on the `parallel` feature toggle of the `gix-features` crate."] pub fn to_cache (self : & OwnShared < Self >) -> crate :: Cache < super :: Handle < OwnShared < super :: Store > > > { self . to_handle () . into () } # [doc = " Create a new cache filled with a handle to this store if this store is held in an `Arc`."] pub fn to_cache_arc (self : & Arc < Self >) -> crate :: Cache < super :: Handle < Arc < super :: Store > > > { self . to_handle_arc () . into () } # [doc = " Create a new database handle to this store if this store is supporting shared ownership."] # [doc = ""] # [doc = " See also, [`to_cache()`][super::Store::to_cache()] which is probably more useful."] pub fn to_handle (self : & OwnShared < Self >) -> super :: Handle < OwnShared < super :: Store > > { let token = self . register_handle () ; super :: Handle { store : self . clone () , refresh : RefreshMode :: default () , ignore_replacements : false , token : Some (token) , inflate : RefCell :: new (Default :: default ()) , snapshot : RefCell :: new (self . collect_snapshot ()) , max_recursion_depth : Self :: INITIAL_MAX_RECURSION_DEPTH , packed_object_count : Default :: default () , } } # [doc = " Create a new database handle to this store if this store is held in an `Arc`."] # [doc = ""] # [doc = " This method is useful in applications that know they will use threads."] pub fn to_handle_arc (self : & Arc < Self >) -> super :: Handle < Arc < super :: Store > > { let token = self . register_handle () ; super :: Handle { store : self . clone () , refresh : Default :: default () , ignore_replacements : false , token : Some (token) , inflate : RefCell :: new (Default :: default ()) , snapshot : RefCell :: new (self . collect_snapshot ()) , max_recursion_depth : Self :: INITIAL_MAX_RECURSION_DEPTH , packed_object_count : Default :: default () , } } # [doc = " Transform the only instance into an `Arc<Self>` or panic if this is not the only Rc handle"] # [doc = " to the contained store."] # [doc = ""] # [doc = " This is meant to be used when the `gix_features::threading::OwnShared` refers to an `Rc` as it was compiled without the"] # [doc = " `parallel` feature toggle."] pub fn into_shared_arc (self : OwnShared < Self >) -> Arc < Self > { match OwnShared :: try_unwrap (self) { Ok (this) => Arc :: new (this) , Err (_) => panic ! ("BUG: Must be called when there is only one owner for this RC") , } } }
    };
}

impl_65!()