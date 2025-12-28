macro_rules! deps {
    () => {
        PrepareCheckout!();
    };
}

macro_rules! impl_98 {
    () => {
        deps!();
        impl Drop for PrepareCheckout { fn drop (& mut self) { if let Some (repo) = self . repo . take () { std :: fs :: remove_dir_all (repo . workdir () . unwrap_or_else (| | repo . path ())) . ok () ; } } }
    };
}

impl_98!()