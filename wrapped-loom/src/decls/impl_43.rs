macro_rules! deps {
    () => {
        FirstSeen!();
        Set!();
    };
}

macro_rules! impl_43 {
    () => {
        deps!();
        impl FirstSeen { fn new () -> FirstSeen { FirstSeen ([u16 :: max_value () ; MAX_THREADS]) } fn touch (& mut self , threads : & thread :: Set) { if self . 0 [threads . active_id () . as_usize ()] == u16 :: max_value () { self . 0 [threads . active_id () . as_usize ()] = threads . active_atomic_version () ; } } fn is_seen_by_current (& self , threads : & thread :: Set) -> bool { for (thread_id , version) in threads . active () . causality . versions (threads . execution_id ()) { match self . 0 [thread_id . as_usize ()] { u16 :: MAX => { } v if v <= version => return true , _ => { } } } false } fn is_seen_before_yield (& self , threads : & thread :: Set) -> bool { let thread_id = threads . active_id () ; let last_yield = match threads . active () . last_yield { Some (v) => v , None => return false , } ; match self . 0 [thread_id . as_usize ()] { u16 :: MAX => false , v => v <= last_yield , } } }
    };
}

impl_43!()