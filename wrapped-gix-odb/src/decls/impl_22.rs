macro_rules! deps {
    () => {
        Store!();
        State!();
        Error!();
        AllObjects!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl AllObjects { # [doc = " Create a new iterator from a dynamic store, which will be forced to load all indices eagerly and in the current thread."] pub fn new (db : & dynamic :: Store) -> Result < Self , crate :: store :: load_index :: Error > { let snapshot = db . load_all_indices () ? ; let packed_objects = snapshot . indices . iter () . fold (0usize , | dbc , index | dbc . saturating_add (index . num_objects () as usize)) ; let mut index_iter = snapshot . indices . into_iter () ; let loose_dbs = snapshot . loose_dbs ; let order = Default :: default () ; let state = match index_iter . next () { Some (index) => { let num_objects = index . num_objects () ; State :: Pack { index_iter , ordered_entries : maybe_sort_entries (& index , order) , index , entry_index : 0 , num_objects , } } None => { let index = 0 ; State :: Loose { iter : loose_dbs . get (index) . expect ("at least one loose db") . iter () , index , } } } ; Ok (AllObjects { state , loose_dbs , num_objects : packed_objects , order , }) } }
    };
}

impl_22!();