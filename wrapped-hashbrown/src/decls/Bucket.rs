macro_rules! Bucket {
    () => {
        # [doc = " A reference to a hash table bucket containing a `T`."] # [doc = ""] # [doc = " This is usually just a pointer to the element itself. However if the element"] # [doc = " is a ZST, then we instead track the index of the element in the table so"] # [doc = " that `erase` works properly."] pub struct Bucket < T > { ptr : NonNull < T > , }
    };
}

Bucket!()