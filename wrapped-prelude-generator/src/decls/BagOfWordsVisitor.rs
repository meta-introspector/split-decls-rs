macro_rules! BagOfWordsVisitor {
    () => {
        pub struct BagOfWordsVisitor { pub bag_of_words : HashMap < String , usize > , }
    };
}

BagOfWordsVisitor!();