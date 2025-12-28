macro_rules! deps {
    () => {
        Collect!();
        EntryRef!();
        Delegate!();
        Action!();
        Status!();
        Item!();
    };
}

macro_rules! impl_120 {
    () => {
        deps!();
        impl gix_dir :: walk :: Delegate for Collect { fn emit (& mut self , entry : gix_dir :: EntryRef < '_ > , collapsed_directory_status : Option < gix_dir :: entry :: Status > ,) -> gix_dir :: walk :: Action { let item = Item :: new (entry , collapsed_directory_status) ; # [cfg (feature = "parallel")] self . tx . send (item) . ok () ; # [cfg (not (feature = "parallel"))] self . items . push (item) ; gix_dir :: walk :: Action :: Continue } }
    };
}

impl_120!();