macro_rules! deps {
    () => {
        TreeEntry!();
    };
}

macro_rules! impl_832 {
    () => {
        deps!();
        impl < 'a > Ord for TreeEntry < 'a > { fn cmp (& self , other : & TreeEntry < 'a >) -> Ordering { c_cmp_to_ordering (unsafe { raw :: git_tree_entry_cmp (& * self . raw () , & * other . raw ()) }) } }
    };
}

impl_832!();