macro_rules! deps {
    () => {
        SequenceId!();
        InOrderIter!();
    };
}

macro_rules! impl_45 {
    () => {
        deps!();
        impl < T , E , I > Iterator for InOrderIter < T , I > where I : Iterator < Item = Result < (SequenceId , T) , E > > , { type Item = Result < T , E > ; fn next (& mut self) -> Option < Self :: Item > { if self . is_done { return None ; } 'find_next_in_sequence : loop { match self . inner . next () { Some (Ok ((c , v))) => match c . cmp (& self . next_chunk) { Ordering :: Equal => { self . next_chunk += 1 ; return Some (Ok (v)) ; } Ordering :: Less => { unreachable ! ("in a correctly ordered sequence we can never see keys again, got {}" , c) } Ordering :: Greater => { let previous = self . store . insert (c , v) ; assert ! (previous . is_none () , "Chunks are returned only once, input is an invalid sequence") ; if let Some (v) = self . store . remove (& self . next_chunk) { self . next_chunk += 1 ; return Some (Ok (v)) ; } continue 'find_next_in_sequence ; } } , Some (Err (e)) => { self . is_done = true ; self . store . clear () ; return Some (Err (e)) ; } None => match self . store . remove (& self . next_chunk) { Some (v) => { self . next_chunk += 1 ; return Some (Ok (v)) ; } None => { debug_assert ! (self . store . is_empty () , "When iteration is done we should not have stored items left") ; return None ; } } , } } } }
    };
}

impl_45!();