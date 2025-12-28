macro_rules! deps {
    () => {
        IndexMap!();
    };
}

macro_rules! FnvIndexMap {
    () => {
        deps!();
        # [doc = " An [`IndexMap`] using the default FNV hasher."] # [doc = ""] # [doc = " A list of all Methods and Traits available for `FnvIndexMap` can be found in"] # [doc = " the [`IndexMap`] documentation."] # [doc = ""] # [doc = " # Examples"] # [doc = " ```"] # [doc = " use heapless::index_map::FnvIndexMap;"] # [doc = ""] # [doc = " // A hash map with a capacity of 16 key-value pairs allocated on the stack"] # [doc = " let mut book_reviews = FnvIndexMap::<_, _, 16>::new();"] # [doc = ""] # [doc = " // review some books."] # [doc = " book_reviews"] # [doc = "     .insert(\"Adventures of Huckleberry Finn\", \"My favorite book.\")"] # [doc = "     .unwrap();"] # [doc = " book_reviews"] # [doc = "     .insert(\"Grimms' Fairy Tales\", \"Masterpiece.\")"] # [doc = "     .unwrap();"] # [doc = " book_reviews"] # [doc = "     .insert(\"Pride and Prejudice\", \"Very enjoyable.\")"] # [doc = "     .unwrap();"] # [doc = " book_reviews"] # [doc = "     .insert(\"The Adventures of Sherlock Holmes\", \"Eye lyked it alot.\")"] # [doc = "     .unwrap();"] # [doc = ""] # [doc = " // check for a specific one."] # [doc = " if !book_reviews.contains_key(\"Les Misérables\") {"] # [doc = "     println!("] # [doc = "         \"We've got {} reviews, but Les Misérables ain't one.\","] # [doc = "         book_reviews.len()"] # [doc = "     );"] # [doc = " }"] # [doc = ""] # [doc = " // oops, this review has a lot of spelling mistakes, let's delete it."] # [doc = " book_reviews.remove(\"The Adventures of Sherlock Holmes\");"] # [doc = ""] # [doc = " // look up the values associated with some keys."] # [doc = " let to_find = [\"Pride and Prejudice\", \"Alice's Adventure in Wonderland\"];"] # [doc = " for book in &to_find {"] # [doc = "     match book_reviews.get(book) {"] # [doc = "         Some(review) => println!(\"{}: {}\", book, review),"] # [doc = "         None => println!(\"{} is unreviewed.\", book),"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " // iterate over everything."] # [doc = " for (book, review) in &book_reviews {"] # [doc = "     println!(\"{}: \\\"{}\\\"\", book, review);"] # [doc = " }"] # [doc = " ```"] pub type FnvIndexMap < K , V , const N : usize > = IndexMap < K , V , BuildHasherDefault < FnvHasher > , N > ;
    };
}

FnvIndexMap!()