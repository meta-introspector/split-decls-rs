macro_rules! deps {
    () => {
        WriteBatch!();
    };
}

macro_rules! WriteBatchIterator {
    () => {
        deps!();
        # [doc = " Receives the puts and deletes of a write batch."] # [doc = ""] # [doc = " The application must provide an implementation of this trait when"] # [doc = " iterating the operations within a `WriteBatch`"] pub trait WriteBatchIterator { # [doc = " Called with a key and value that were `put` into the batch."] fn put (& mut self , key : & [u8] , value : & [u8]) ; # [doc = " Called with a key that was `delete`d from the batch."] fn delete (& mut self , key : & [u8]) ; }
    };
}

WriteBatchIterator!()