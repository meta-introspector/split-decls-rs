macro_rules! deps {
    () => {
        Binding!();
        TreeEntry!();
    };
}

macro_rules! impl_830 {
    () => {
        deps!();
        impl < 'a > Clone for TreeEntry < 'a > { fn clone (& self) -> TreeEntry < 'a > { let mut ret = ptr :: null_mut () ; unsafe { assert_eq ! (raw :: git_tree_entry_dup (& mut ret , &* self . raw ()) , 0) ; Binding :: from_raw (ret) } } }
    };
}

impl_830!()