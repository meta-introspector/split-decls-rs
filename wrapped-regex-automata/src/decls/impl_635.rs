macro_rules! deps {
    () => {
        GroupInfoAllNames!();
        PatternID!();
    };
}

macro_rules! impl_635 {
    () => {
        deps!();
        impl < 'a > Iterator for GroupInfoAllNames < 'a > { type Item = (PatternID , usize , Option < & 'a str >) ; fn next (& mut self) -> Option < (PatternID , usize , Option < & 'a str >) > { if self . group_info . 0 . index_to_name . is_empty () { return None ; } if self . current_pid . is_none () { self . current_pid = Some (self . pids . next () ?) ; } let pid = self . current_pid . unwrap () ; if self . names . is_none () { self . names = Some (self . group_info . pattern_names (pid) . enumerate ()) ; } let (group_index , name) = match self . names . as_mut () . unwrap () . next () { Some ((group_index , name)) => (group_index , name) , None => { self . current_pid = None ; self . names = None ; return self . next () ; } } ; Some ((pid , group_index , name)) } }
    };
}

impl_635!()