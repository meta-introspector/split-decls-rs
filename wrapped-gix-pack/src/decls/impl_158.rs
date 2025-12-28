macro_rules! deps {
    () => {
        Error!();
        Find!();
        Item!();
        Entry!();
        Header!();
        LookupRefDeltaObjectsIter!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl < I , Find > Iterator for LookupRefDeltaObjectsIter < I , Find > where I : Iterator < Item = Result < input :: Entry , input :: Error > > , Find : gix_object :: Find , { type Item = Result < input :: Entry , input :: Error > ; fn next (& mut self) -> Option < Self :: Item > { if self . error { return None ; } if let Some (delta) = self . next_delta . take () { return Some (Ok (delta)) ; } match self . inner . next () { Some (Ok (mut entry)) => match entry . header { Header :: RefDelta { base_id } => { match self . inserted_entry_length_at_offset . iter () . rfind (| e | e . oid == base_id) { None => { let base_entry = match self . lookup . try_find (& base_id , & mut self . buf) . ok () ? { Some (obj) => { let current_pack_offset = entry . pack_offset ; let mut entry = match input :: Entry :: from_data_obj (& obj , 0) { Ok (e) => e , Err (err) => return Some (Err (err)) , } ; entry . pack_offset = self . shifted_pack_offset (current_pack_offset) ; self . track_change (entry . pack_offset , current_pack_offset , entry . bytes_in_pack () as i64 , Some (base_id) ,) ; entry } None => { self . error = true ; return Some (Err (input :: Error :: NotFound { object_id : base_id })) ; } } ; { self . shift_entry_and_point_to_base_by_offset (& mut entry , base_entry . bytes_in_pack ()) ; self . next_delta = Some (entry) ; } Some (Ok (base_entry)) } Some (base_entry) => { let base_distance = self . shifted_pack_offset (entry . pack_offset) - base_entry . shifted_pack_offset ; self . shift_entry_and_point_to_base_by_offset (& mut entry , base_distance) ; Some (Ok (entry)) } } } _ => { if self . inserted_entries_length_in_bytes != 0 { if let Header :: OfsDelta { base_distance } = entry . header { let base_pack_offset = entry . pack_offset . checked_sub (base_distance) . expect ("distance to be in range of pack") ; match self . inserted_entry_length_at_offset . binary_search_by_key (& base_pack_offset , | c | c . pack_offset) { Ok (index) => { let index = { let maybe_index_of_actual_entry = index + 1 ; self . inserted_entry_length_at_offset . get (maybe_index_of_actual_entry) . and_then (| c | { (c . pack_offset == base_pack_offset) . then_some (maybe_index_of_actual_entry) }) . unwrap_or (index) } ; let new_distance = self . shifted_pack_offset (entry . pack_offset) . checked_sub (self . inserted_entry_length_at_offset [index] . shifted_pack_offset) . expect ("a base that is behind us in the pack") ; self . shift_entry_and_point_to_base_by_offset (& mut entry , new_distance) ; } Err (index) => { let change_since_offset = self . inserted_entry_length_at_offset [index ..] . iter () . map (| c | c . size_change_in_bytes) . sum :: < i64 > () ; let new_distance : u64 = { (base_distance as i64 + change_since_offset) . try_into () . expect ("it still points behind us") } ; self . shift_entry_and_point_to_base_by_offset (& mut entry , new_distance) ; } } } else { entry . pack_offset = self . shifted_pack_offset (entry . pack_offset) ; } } Some (Ok (entry)) } } , other => other , } } fn size_hint (& self) -> (usize , Option < usize >) { let (min , max) = self . inner . size_hint () ; max . map_or_else (| | (min * 2 , None) , | max | (min , Some (max * 2))) } }
    };
}

impl_158!()