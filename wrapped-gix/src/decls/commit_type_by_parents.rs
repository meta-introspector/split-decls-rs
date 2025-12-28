macro_rules! commit_type_by_parents {
    () => {
        pub (crate) fn commit_type_by_parents (count : usize) -> Option < & 'static str > { Some (match count { 0 => "initial" , 1 => return None , _two_or_more => "merge" , }) }
    };
}

commit_type_by_parents!();