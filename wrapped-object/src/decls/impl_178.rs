macro_rules! deps {
    () => {
        Members!();
        Result!();
        ArchiveMemberIterator!();
        Item!();
        ReadRef!();
        ArchiveMember!();
    };
}

macro_rules! impl_178 {
    () => {
        deps!();
        impl < 'data , R : ReadRef < 'data > > Iterator for ArchiveMemberIterator < 'data , R > { type Item = read :: Result < ArchiveMember < 'data > > ; fn next (& mut self) -> Option < Self :: Item > { match & mut self . members { Members :: Common { ref mut offset , ref mut end_offset , } => { if * offset >= * end_offset { return None ; } let member = ArchiveMember :: parse (self . data , offset , self . names , self . thin) ; if member . is_err () { * offset = * end_offset ; } Some (member) } Members :: AixBig { ref mut index } => match * * index { [] => None , [ref first , ref rest @ ..] => { * index = rest ; let member = ArchiveMember :: parse_aixbig_index (self . data , first) ; if member . is_err () { * index = & [] ; } Some (member) } } , } } }
    };
}

impl_178!();