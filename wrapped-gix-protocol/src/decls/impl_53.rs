macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! impl_53 {
    () => {
        deps!();
        impl Context { fn aggregate_refspecs (& self) -> Vec < gix_refspec :: RefSpec > { let mut all_refspecs = self . fetch_refspecs . clone () ; all_refspecs . extend (self . extra_refspecs . iter () . cloned ()) ; all_refspecs } }
    };
}

impl_53!()