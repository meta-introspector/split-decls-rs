macro_rules! deps {
    () => {
        PrepareFetch!();
    };
}

macro_rules! impl_92 {
    () => {
        deps!();
        impl Drop for PrepareFetch { fn drop (& mut self) { if let Some (repo) = self . repo . take () { std :: fs :: remove_dir_all (repo . workdir () . unwrap_or_else (| | repo . path ())) . ok () ; } } }
    };
}

impl_92!();