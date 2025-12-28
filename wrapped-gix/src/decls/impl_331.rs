macro_rules! deps {
    () => {
        Repository!();
    };
}

macro_rules! impl_331 {
    () => {
        deps!();
        impl PartialEq < crate :: Repository > for crate :: Repository { fn eq (& self , other : & crate :: Repository) -> bool { self . git_dir () . canonicalize () . ok () == other . git_dir () . canonicalize () . ok () && self . work_tree . as_deref () . and_then (| wt | wt . canonicalize () . ok ()) == other . work_tree . as_deref () . and_then (| wt | wt . canonicalize () . ok ()) } }
    };
}

impl_331!();