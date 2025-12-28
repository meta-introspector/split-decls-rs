macro_rules! deps {
    () => {
        Options!();
        ColumnFamilyTtl!();
    };
}

macro_rules! ColumnFamilyDescriptor {
    () => {
        deps!();
        # [doc = " A descriptor for a RocksDB column family."] # [doc = ""] # [doc = " A description of the column family, containing the name and `Options`."] pub struct ColumnFamilyDescriptor { pub (crate) name : String , pub (crate) options : Options , pub (crate) ttl : ColumnFamilyTtl , }
    };
}

ColumnFamilyDescriptor!();