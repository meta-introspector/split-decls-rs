macro_rules! deps {
    () => {
        SectionId!();
    };
}

macro_rules! find_insert_pos_by_order {
    () => {
        deps!();
        fn find_insert_pos_by_order (sections_with_name : & [SectionId] , before_order : usize , lookup_section_order : impl Fn (SectionId) -> usize ,) -> usize { let mut insert_pos = sections_with_name . len () ; for (idx , candidate_id) in sections_with_name . iter () . enumerate () { let candidate_order = lookup_section_order (* candidate_id) ; match candidate_order . cmp (& before_order) { Ordering :: Less => { } Ordering :: Equal => { insert_pos = idx + 1 ; break ; } Ordering :: Greater => { insert_pos = idx ; break ; } } } insert_pos }
    };
}

find_insert_pos_by_order!();