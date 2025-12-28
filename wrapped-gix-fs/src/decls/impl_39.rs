macro_rules! deps {
    () => {
        Iter!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl < 'a > Iterator for Iter < 'a > { type Item = std :: io :: Result < & 'a Path > ; fn next (& mut self) -> Option < Self :: Item > { match self . cursor . take () { Some (dir) => { let next = match std :: fs :: remove_dir (dir) { Ok (()) => Some (Ok (dir)) , Err (err) => match err . kind () { std :: io :: ErrorKind :: NotFound => Some (Ok (dir)) , _other_error_kind => return Some (Err (err)) , } , } ; self . cursor = match dir . parent () { Some (parent) => (parent != self . boundary) . then_some (parent) , None => { unreachable ! ("directory {:?} ran out of parents, this really shouldn't happen before hitting the boundary {:?}" , dir , self . boundary) } } ; next } None => None , } } }
    };
}

impl_39!()