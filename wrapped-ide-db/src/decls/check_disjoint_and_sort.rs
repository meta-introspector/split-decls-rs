macro_rules! deps {
    () => {
        Indel!();
    };
}

macro_rules! check_disjoint_and_sort {
    () => {
        deps!();
        fn check_disjoint_and_sort (indels : & mut [Indel]) -> bool { indels . sort_by_key (| indel | (indel . delete . start () , indel . delete . end ())) ; check_disjoint (& mut indels . iter ()) }
    };
}

check_disjoint_and_sort!()