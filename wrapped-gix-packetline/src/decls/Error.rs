macro_rules! Error {
    () => {
        # [doc = " The error returned by most functions in the [`encode`](crate::encode) module"] # [derive (Debug , thiserror :: Error)] # [allow (missing_docs)] pub enum Error { # [error ("Cannot encode more than {MAX_DATA_LEN} bytes, got {length_in_bytes}")] DataLengthLimitExceeded { length_in_bytes : usize } , # [error ("Empty lines are invalid")] DataIsEmpty , }
    };
}

Error!();