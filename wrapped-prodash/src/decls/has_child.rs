macro_rules! deps {
    () => {
        Key!();
        Task!();
    };
}

macro_rules! has_child {
    () => {
        deps!();
        fn has_child (entries : & [(Key , Task)] , index : usize) -> bool { entries . get (index + 1) . and_then (| (other_key , other_val) | { entries . get (index) . map (| (cur_key , _) | { cur_key . shares_parent_with (other_key , cur_key . level ()) && other_val . progress . is_some () }) }) . unwrap_or (false) }
    };
}

has_child!();