macro_rules! deps {
    () => {
        FileChange!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl fmt :: Debug for FileChange { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { let mut d = fmt . debug_struct ("Change") ; if let Some (roots) = & self . roots { d . field ("roots" , roots) ; } if ! self . files_changed . is_empty () { d . field ("files_changed" , & self . files_changed . len ()) ; } if self . crate_graph . is_some () { d . field ("crate_graph" , & self . crate_graph) ; } d . finish () } }
    };
}

impl_1!()