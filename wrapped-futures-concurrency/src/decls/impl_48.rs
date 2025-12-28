macro_rules! deps {
    () => {
        PollState!();
        PollVec!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl PollVec { pub (crate) fn new (len : usize) -> Self { Self (smallvec ! [PollState :: None ; len]) } pub (crate) fn new_pending (len : usize) -> Self { Self (smallvec ! [PollState :: Pending ; len]) } # [doc = " Get an iterator of indexes of all items which are \"ready\"."] pub (crate) fn ready_indexes (& self) -> impl Iterator < Item = usize > + '_ { self . iter () . cloned () . enumerate () . filter (| (_ , state) | state . is_ready ()) . map (| (i , _) | i) } # [doc = " Get an iterator of indexes of all items which are \"pending\"."] # [allow (unused)] pub (crate) fn pending_indexes (& self) -> impl Iterator < Item = usize > + '_ { self . iter () . cloned () . enumerate () . filter (| (_ , state) | state . is_pending ()) . map (| (i , _) | i) } # [doc = " Get an iterator of indexes of all items which are \"consumed\"."] # [allow (unused)] pub (crate) fn consumed_indexes (& self) -> impl Iterator < Item = usize > + '_ { self . iter () . cloned () . enumerate () . filter (| (_ , state) | state . is_none ()) . map (| (i , _) | i) } # [doc = " Mark all items as \"pending\""] # [inline] pub (crate) fn set_all_pending (& mut self) { self . 0 . fill (PollState :: Pending) ; } # [doc = " Mark all items as \"none\""] # [inline] # [allow (unused)] pub (crate) fn set_all_none (& mut self) { self . 0 . fill (PollState :: None) ; } # [doc = " Resize the `PollVec`"] pub (crate) fn resize (& mut self , len : usize) { self . 0 . resize_with (len , | | PollState :: None) } }
    };
}

impl_48!();