macro_rules! deps {
    () => {
        State!();
        AllObjects!();
        Error!();
        Store!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Iterator for AllObjects { type Item = Result < ObjectId , loose :: iter :: Error > ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . state { State :: Depleted => None , State :: Pack { index_iter , ordered_entries , index , entry_index , num_objects , } => { if * entry_index < * num_objects { let oid = match ordered_entries { Some (entries) => index . oid_at_index (entries [* entry_index as usize] . entry_index) , None => index . oid_at_index (* entry_index) , } . to_owned () ; * entry_index += 1 ; Some (Ok (oid)) } else { match index_iter . next () { Some (new_index) => { * ordered_entries = maybe_sort_entries (& new_index , self . order) ; * index = new_index ; * entry_index = 0 ; * num_objects = index . num_objects () ; } None => { let index = 0 ; self . state = State :: Loose { iter : self . loose_dbs . get (index) . expect ("at least one loose odb") . iter () , index , } } } self . next () } } State :: Loose { iter , index } => match iter . next () { Some (id) => Some (id) , None => { * index += 1 ; match self . loose_dbs . get (* index) . map (loose :: Store :: iter) { Some (new_iter) => { * iter = new_iter ; self . next () } None => { self . state = State :: Depleted ; None } } } } , } } fn size_hint (& self) -> (usize , Option < usize >) { (self . num_objects , None) } }
    };
}

impl_24!()