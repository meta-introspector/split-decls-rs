macro_rules! deps {
    () => {
        Transition!();
        SparseTransitionIter!();
    };
}

macro_rules! impl_77 {
    () => {
        deps!();
        impl < 'a > Iterator for SparseTransitionIter < 'a > { type Item = (u8 , u8 , Transition) ; fn next (& mut self) -> Option < (u8 , u8 , Transition) > { while let Some ((b , & trans)) = self . it . next () { let b = b . as_u8 () ; let (prev_start , prev_end , prev_trans) = match self . cur { Some (t) => t , None => { self . cur = Some ((b , b , trans)) ; continue ; } } ; if prev_trans == trans { self . cur = Some ((prev_start , b , prev_trans)) ; } else { self . cur = Some ((b , b , trans)) ; if prev_trans . state_id () != DEAD { return Some ((prev_start , prev_end , prev_trans)) ; } } } if let Some ((start , end , trans)) = self . cur . take () { if trans . state_id () != DEAD { return Some ((start , end , trans)) ; } } None } }
    };
}

impl_77!();