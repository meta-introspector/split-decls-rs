macro_rules! deps {
    () => {
        RawTable!();
        HashMap!();
        HashSet!();
    };
}

macro_rules! HashTable {
    () => {
        deps!();
        # [doc = " Low-level hash table with explicit hashing."] # [doc = ""] # [doc = " The primary use case for this type over [`HashMap`] or [`HashSet`] is to"] # [doc = " support types that do not implement the [`Hash`] and [`Eq`] traits, but"] # [doc = " instead require additional data not contained in the key itself to compute a"] # [doc = " hash and compare two elements for equality."] # [doc = ""] # [doc = " Examples of when this can be useful include:"] # [doc = " - An `IndexMap` implementation where indices into a `Vec` are stored as"] # [doc = "   elements in a `HashTable<usize>`. Hashing and comparing the elements"] # [doc = "   requires indexing the associated `Vec` to get the actual value referred to"] # [doc = "   by the index."] # [doc = " - Avoiding re-computing a hash when it is already known."] # [doc = " - Mutating the key of an element in a way that doesn't affect its hash."] # [doc = ""] # [doc = " To achieve this, `HashTable` methods that search for an element in the table"] # [doc = " require a hash value and equality function to be explicitly passed in as"] # [doc = " arguments. The method will then iterate over the elements with the given"] # [doc = " hash and call the equality function on each of them, until a match is found."] # [doc = ""] # [doc = " In most cases, a `HashTable` will not be exposed directly in an API. It will"] # [doc = " instead be wrapped in a helper type which handles the work of calculating"] # [doc = " hash values and comparing elements."] # [doc = ""] # [doc = " Due to its low-level nature, this type provides fewer guarantees than"] # [doc = " [`HashMap`] and [`HashSet`]. Specifically, the API allows you to shoot"] # [doc = " yourself in the foot by having multiple elements with identical keys in the"] # [doc = " table. The table itself will still function correctly and lookups will"] # [doc = " arbitrarily return one of the matching elements. However you should avoid"] # [doc = " doing this because it changes the runtime of hash table operations from"] # [doc = " `O(1)` to `O(k)` where `k` is the number of duplicate entries."] # [doc = ""] # [doc = " [`HashMap`]: super::HashMap"] # [doc = " [`HashSet`]: super::HashSet"] # [doc = " [`Eq`]: https://doc.rust-lang.org/std/cmp/trait.Eq.html"] # [doc = " [`Hash`]: https://doc.rust-lang.org/std/hash/trait.Hash.html"] pub struct HashTable < T , A = Global > where A : Allocator , { pub (crate) raw : RawTable < T , A > , }
    };
}

HashTable!();