macro_rules! is_dot_hfs {
    () => {
        fn is_dot_hfs (input : & BStr , search_case_insensitive : & str) -> bool { let mut input = input . chars () . filter (| c | match * c as u32 { 0x200c | 0x200d | 0x200e | 0x200f | 0x202a | 0x202b | 0x202c | 0x202d | 0x202e | 0x206a | 0x206b | 0x206c | 0x206d | 0x206e | 0x206f | 0xfeff => false , _ => true }) ; if input . next () != Some ('.') { return false ; } let mut comp = search_case_insensitive . chars () ; loop { match (comp . next () , input . next ()) { (Some (a) , Some (b)) => { if ! a . eq_ignore_ascii_case (& b) { return false ; } } (None , None) => return true , _ => return false , } } }
    };
}

is_dot_hfs!();