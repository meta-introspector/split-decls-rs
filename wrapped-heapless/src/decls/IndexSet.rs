macro_rules! deps {
    () => {
        IndexMap!();
        FnvIndexSet!();
    };
}

macro_rules! IndexSet {
    () => {
        deps!();
        # [doc = " Fixed capacity [`IndexSet`](https://docs.rs/indexmap/2/indexmap/set/struct.IndexSet.html)."] # [doc = ""] # [doc = " Note that you cannot use `IndexSet` directly, since it is generic around the hashing algorithm"] # [doc = " in use. Pick a concrete instantiation like [`FnvIndexSet`] instead"] # [doc = " or create your own."] # [doc = ""] # [doc = " Note that the capacity of the `IndexSet` must be a power of 2."] # [doc = ""] # [doc = " # Examples"] # [doc = " Since `IndexSet` cannot be used directly, we're using its `FnvIndexSet` instantiation"] # [doc = " for this example."] # [doc = ""] # [doc = " ```"] # [doc = " use heapless::index_set::FnvIndexSet;"] # [doc = ""] # [doc = " // A hash set with a capacity of 16 elements allocated on the stack"] # [doc = " let mut books = FnvIndexSet::<_, 16>::new();"] # [doc = ""] # [doc = " // Add some books."] # [doc = " books.insert(\"A Dance With Dragons\").unwrap();"] # [doc = " books.insert(\"To Kill a Mockingbird\").unwrap();"] # [doc = " books.insert(\"The Odyssey\").unwrap();"] # [doc = " books.insert(\"The Great Gatsby\").unwrap();"] # [doc = ""] # [doc = " // Check for a specific one."] # [doc = " if !books.contains(\"The Winds of Winter\") {"] # [doc = "     println!("] # [doc = "         \"We have {} books, but The Winds of Winter ain't one.\","] # [doc = "         books.len()"] # [doc = "     );"] # [doc = " }"] # [doc = ""] # [doc = " // Remove a book."] # [doc = " books.remove(\"The Odyssey\");"] # [doc = ""] # [doc = " // Iterate over everything."] # [doc = " for book in &books {"] # [doc = "     println!(\"{}\", book);"] # [doc = " }"] # [doc = " ```"] # [cfg_attr (feature = "zeroize" , derive (Zeroize) , zeroize (bound = "T: Zeroize"))] pub struct IndexSet < T , S , const N : usize > { map : IndexMap < T , () , S , N > , }
    };
}

IndexSet!();