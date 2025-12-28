macro_rules! deps {
    () => {
        ColumnFamilyDescriptor!();
        DB!();
        Options!();
        ColumnFamilyTtl!();
    };
}

macro_rules! impl_41 {
    () => {
        deps!();
        impl ColumnFamilyDescriptor { # [doc = " Create a new column family descriptor with the specified name and options."] # [doc = " *WARNING*:"] # [doc = " Will use [`ColumnFamilyTtl::SameAsDb`] as ttl."] pub fn new < S > (name : S , options : Options) -> Self where S : Into < String > , { Self { name : name . into () , options , ttl : ColumnFamilyTtl :: SameAsDb , } } # [doc = " Create a new column family descriptor with the specified name, options, and ttl."] # [doc = " *WARNING*:"] # [doc = " The ttl is applied only when DB is opened with [`crate::db::DB::open_with_ttl()`]."] pub fn new_with_ttl < S > (name : S , options : Options , ttl : ColumnFamilyTtl) -> Self where S : Into < String > , { Self { name : name . into () , options , ttl , } } # [doc = " Sets ttl for the column family. It's applied only when DB is opened with"] # [doc = " [`crate::db::DB::open_with_ttl()`]. Changing ttl after DB is opened has no effect."] pub fn set_ttl (& mut self , ttl : ColumnFamilyTtl) { self . ttl = ttl ; } # [doc = " Get the name of the ColumnFamilyDescriptor."] pub fn name (& self) -> & str { & self . name } pub fn ttl (& self) -> ColumnFamilyTtl { self . ttl } }
    };
}

impl_41!();