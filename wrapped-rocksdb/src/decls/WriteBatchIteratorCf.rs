macro_rules! deps {
    () => {
        WriteBatch!();
    };
}

macro_rules! WriteBatchIteratorCf {
    () => {
        deps!();
        # [doc = " Receives the puts, deletes, and merges of a write batch with column family"] # [doc = " information."] # [doc = ""] # [doc = " This trait extends write batch iteration to support column family-specific"] # [doc = " operations. The application must implement this trait when iterating"] # [doc = " operations within a WriteBatch that contains column family-aware writes."] # [doc = ""] # [doc = " Note that for the default column family \"default\", the column family ID is 0."] pub trait WriteBatchIteratorCf { # [doc = " Called with a column family ID, key, and value that were put into"] # [doc = " the specific column family of the batch."] fn put_cf (& mut self , cf_id : u32 , key : & [u8] , value : & [u8]) ; # [doc = " Called with a column family ID and key that were `delete`d from the"] # [doc = " specific column family of the batch."] fn delete_cf (& mut self , cf_id : u32 , key : & [u8]) ; # [doc = " Called with a column family ID, key, and value that were `merge`d into"] # [doc = " the specific column family of the batch."] # [doc = " Merge operations combine the provided value with the existing value at"] # [doc = " the key using a database-defined merge operator."] fn merge_cf (& mut self , cf_id : u32 , key : & [u8] , value : & [u8]) ; }
    };
}

WriteBatchIteratorCf!();