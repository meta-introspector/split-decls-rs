macro_rules! deps {
    () => {
        PreferenceTrie!();
        Result!();
        State!();
        Literal!();
        Seq!();
    };
}

macro_rules! impl_173 {
    () => {
        deps!();
        impl PreferenceTrie { # [doc = " Minimizes the given sequence of literals while preserving preference"] # [doc = " order semantics."] # [doc = ""] # [doc = " When `keep_exact` is true, the exactness of every literal retained is"] # [doc = " kept. This is useful when dealing with a fully extracted `Seq` that"] # [doc = " only contains exact literals. In that case, we can keep all retained"] # [doc = " literals as exact because we know we'll never need to match anything"] # [doc = " after them and because any removed literals are guaranteed to never"] # [doc = " match."] fn minimize (literals : & mut Vec < Literal > , keep_exact : bool) { let mut trie = PreferenceTrie { states : vec ! [] , matches : vec ! [] , next_literal_index : 1 , } ; let mut make_inexact = vec ! [] ; literals . retain_mut (| lit | match trie . insert (lit . as_bytes ()) { Ok (_) => true , Err (i) => { if ! keep_exact { make_inexact . push (i . checked_sub (1) . unwrap ()) ; } false } }) ; for i in make_inexact { literals [i] . make_inexact () ; } } # [doc = " Returns `Ok` if the given byte string is accepted into this trie and"] # [doc = " `Err` otherwise. The index for the success case corresponds to the"] # [doc = " index of the literal added. The index for the error case corresponds to"] # [doc = " the index of the literal already in the trie that prevented the given"] # [doc = " byte string from being added. (Which implies it is a prefix of the one"] # [doc = " given.)"] # [doc = ""] # [doc = " In short, the byte string given is accepted into the trie if and only"] # [doc = " if it is possible for it to match when executing a preference order"] # [doc = " search."] fn insert (& mut self , bytes : & [u8]) -> Result < usize , usize > { let mut prev = self . root () ; if let Some (idx) = self . matches [prev] { return Err (idx . get ()) ; } for & b in bytes . iter () { match self . states [prev] . trans . binary_search_by_key (& b , | t | t . 0) { Ok (i) => { prev = self . states [prev] . trans [i] . 1 ; if let Some (idx) = self . matches [prev] { return Err (idx . get ()) ; } } Err (i) => { let next = self . create_state () ; self . states [prev] . trans . insert (i , (b , next)) ; prev = next ; } } } let idx = self . next_literal_index ; self . next_literal_index += 1 ; self . matches [prev] = NonZeroUsize :: new (idx) ; Ok (idx) } # [doc = " Returns the root state ID, and if it doesn't exist, creates it."] fn root (& mut self) -> usize { if ! self . states . is_empty () { 0 } else { self . create_state () } } # [doc = " Creates a new empty state and returns its ID."] fn create_state (& mut self) -> usize { let id = self . states . len () ; self . states . push (State :: default ()) ; self . matches . push (None) ; id } }
    };
}

impl_173!()