macro_rules! deps {
    () => {
        IndexSet!();
    };
}

macro_rules! FnvIndexSet {
    () => {
        deps!();
        # [doc = " An [`IndexSet`] using the default FNV hasher."] # [doc = ""] # [doc = " A list of all Methods and Traits available for `FnvIndexSet` can be found in"] # [doc = " the [`IndexSet`] documentation."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use heapless::index_set::FnvIndexSet;"] # [doc = ""] # [doc = " // A hash set with a capacity of 16 elements allocated on the stack"] # [doc = " let mut books = FnvIndexSet::<_, 16>::new();"] # [doc = ""] # [doc = " // Add some books."] # [doc = " books.insert(\"A Dance With Dragons\").unwrap();"] # [doc = " books.insert(\"To Kill a Mockingbird\").unwrap();"] # [doc = " books.insert(\"The Odyssey\").unwrap();"] # [doc = " books.insert(\"The Great Gatsby\").unwrap();"] # [doc = ""] # [doc = " // Check for a specific one."] # [doc = " if !books.contains(\"The Winds of Winter\") {"] # [doc = "     println!("] # [doc = "         \"We have {} books, but The Winds of Winter ain't one.\","] # [doc = "         books.len()"] # [doc = "     );"] # [doc = " }"] # [doc = ""] # [doc = " // Remove a book."] # [doc = " books.remove(\"The Odyssey\");"] # [doc = ""] # [doc = " // Iterate over everything."] # [doc = " for book in &books {"] # [doc = "     println!(\"{}\", book);"] # [doc = " }"] # [doc = " ```"] pub type FnvIndexSet < T , const N : usize > = IndexSet < T , BuildHasherDefault < FnvHasher > , N > ;
    };
}

FnvIndexSet!()