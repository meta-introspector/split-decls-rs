macro_rules! deps {
    () => {
        Store!();
        Iter!();
    };
}

macro_rules! impl_111 {
    () => {
        deps!();
        # [doc = " Iteration and traversal"] impl loose :: Store { # [doc = " Return an iterator over all objects contained in the database."] # [doc = ""] # [doc = " The [`Id`][gix_hash::ObjectId]s returned by the iterator can typically be used in the [`locate(…)`][loose::Store::try_find()] method."] # [doc = " _Note_ that the result is not sorted or stable, thus ordering can change between runs."] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " [`loose::Iter`] is used instead of `impl Iterator<…>` to allow using this iterator in struct fields, as is currently"] # [doc = " needed if iterators need to be implemented by hand in the absence of generators."] pub fn iter (& self) -> loose :: Iter { loose :: Iter { inner : fs :: walkdir_new (& self . path , fs :: walkdir :: Parallelism :: ThreadPoolPerTraversal { thread_name : "gix_odb::loose::Store::iter: fs-walk" , } , false ,) . min_depth (2) . max_depth (3) . follow_links (false) . into_iter () , hash_hex_len : self . object_hash . len_in_hex () , } } }
    };
}

impl_111!()