macro_rules! ColumnFamilyTtl {
    () => {
        # [derive (Debug , Clone , Copy , Default)] # [doc = " Specifies the TTL behavior for a column family."] # [doc = " <https://github.com/facebook/rocksdb/blob/18cecb9c46b4c2a8b148659dac2fcab5a843d32b/include/rocksdb/utilities/db_ttl.h#L16-L46>"] pub enum ColumnFamilyTtl { # [doc = " Will internally set TTL to -1 (disabled)"] # [default] Disabled , # [doc = " Will set ttl to the specified duration"] Duration (Duration) , # [doc = " Will use ttl specified at db open time"] SameAsDb , }
    };
}

ColumnFamilyTtl!()