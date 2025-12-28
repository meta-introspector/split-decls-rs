macro_rules! deps {
    () => {
        BagOfWordsVisitor!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl BagOfWordsVisitor { pub fn new () -> Self { BagOfWordsVisitor { bag_of_words : HashMap :: new () , } } pub fn extract_from_file (file : & syn :: File) -> Self { let mut visitor = Self :: new () ; visitor . visit_file (file) ; visitor } fn add_ident_to_bag (& mut self , ident : & Ident) { for subword in tokenize_ident_to_subwords (& ident . to_string ()) { * self . bag_of_words . entry (subword) . or_insert (0) += 1 ; } } }
    };
}

impl_8!()