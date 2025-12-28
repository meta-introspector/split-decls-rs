macro_rules! deps {
    () => {
        BoundColumnFamily!();
        DBInner!();
        Options!();
        MultiThreaded!();
        DBCommon!();
        Error!();
        UnboundColumnFamily!();
    };
}

macro_rules! impl_115 {
    () => {
        deps!();
        impl < I : DBInner > DBCommon < MultiThreaded , I > { # [doc = " Creates column family with given name and options"] pub fn create_cf < N : AsRef < str > > (& self , name : N , opts : & Options) -> Result < () , Error > { let mut cfs = self . cfs . cfs . write () . unwrap () ; let inner = self . create_inner_cf_handle (name . as_ref () , opts) ? ; cfs . insert (name . as_ref () . to_string () , Arc :: new (UnboundColumnFamily { inner }) ,) ; Ok (()) } # [doc = " Drops the column family with the given name by internally locking the inner column"] # [doc = " family map. This avoids needing `&mut self` reference"] pub fn drop_cf (& self , name : & str) -> Result < () , Error > { if let Some (cf) = self . cfs . cfs . write () . unwrap () . remove (name) { self . drop_column_family (cf . inner , cf) } else { Err (Error :: new (format ! ("Invalid column family: {name}"))) } } # [doc = " Returns the underlying column family handle"] pub fn cf_handle (& self , name : & str) -> Option < Arc < BoundColumnFamily < '_ > > > { self . cfs . cfs . read () . unwrap () . get (name) . cloned () . map (UnboundColumnFamily :: bound_column_family) } }
    };
}

impl_115!();