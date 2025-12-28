macro_rules! deps {
    () => {
        HirDisplayError!();
        HirFormatter!();
        HirDisplay!();
    };
}

macro_rules! impl_483 {
    () => {
        deps!();
        impl < 'db > HirFormatter < '_ , 'db > { pub fn krate (& self) -> Crate { self . display_target . krate } pub fn edition (& self) -> Edition { self . display_target . edition } pub fn write_joined < T : HirDisplay < 'db > > (& mut self , iter : impl IntoIterator < Item = T > , sep : & str ,) -> Result < () , HirDisplayError > { let mut first = true ; for e in iter { if ! first { write ! (self , "{sep}") ? ; } first = false ; if self . should_truncate () { return write ! (self , "{TYPE_HINT_TRUNCATION}") ; } e . hir_fmt (self) ? ; } Ok (()) } # [doc = " This allows using the `write!` macro directly with a `HirFormatter`."] pub fn write_fmt (& mut self , args : fmt :: Arguments < '_ >) -> Result < () , HirDisplayError > { self . buf . clear () ; fmt :: write (& mut self . buf , args) ? ; self . curr_size += self . buf . len () ; self . fmt . write_str (& self . buf) . map_err (HirDisplayError :: from) } pub fn write_str (& mut self , s : & str) -> Result < () , HirDisplayError > { self . fmt . write_str (s) ? ; Ok (()) } pub fn write_char (& mut self , c : char) -> Result < () , HirDisplayError > { self . fmt . write_char (c) ? ; Ok (()) } pub fn should_truncate (& self) -> bool { match self . max_size { Some (max_size) => self . curr_size >= max_size , None => false , } } pub fn omit_verbose_types (& self) -> bool { self . omit_verbose_types } pub fn show_container_bounds (& self) -> bool { self . show_container_bounds } }
    };
}

impl_483!()