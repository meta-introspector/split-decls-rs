macro_rules! deps {
    () => {
        NextInsert!();
        StateID!();
        RangeTrie!();
    };
}

macro_rules! impl_567 {
    () => {
        deps!();
        impl NextInsert { # [doc = " Create the next item to visit. The given state ID should correspond"] # [doc = " to the state at which the first range in the given slice should be"] # [doc = " inserted. The slice given must not be empty and it must be no longer"] # [doc = " than 4."] fn new (state_id : StateID , ranges : & [Utf8Range]) -> NextInsert { let len = ranges . len () ; assert ! (len > 0) ; assert ! (len <= 4) ; let mut tmp = [Utf8Range { start : 0 , end : 0 } ; 4] ; tmp [.. len] . copy_from_slice (ranges) ; NextInsert { state_id , ranges : tmp , len : u8 :: try_from (len) . unwrap () } } # [doc = " Push a new empty state to visit along with any remaining ranges that"] # [doc = " still need to be inserted. The ID of the new empty state is returned."] # [doc = ""] # [doc = " If ranges is empty, then no new state is created and FINAL is returned."] fn push (trie : & mut RangeTrie , stack : & mut Vec < NextInsert > , ranges : & [Utf8Range] ,) -> StateID { if ranges . is_empty () { FINAL } else { let next_id = trie . add_empty () ; stack . push (NextInsert :: new (next_id , ranges)) ; next_id } } # [doc = " Return the ID of the state to visit."] fn state_id (& self) -> StateID { self . state_id } # [doc = " Return the remaining ranges to insert."] fn ranges (& self) -> & [Utf8Range] { & self . ranges [.. usize :: try_from (self . len) . unwrap ()] } }
    };
}

impl_567!()