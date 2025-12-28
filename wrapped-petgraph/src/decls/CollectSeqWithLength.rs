macro_rules! CollectSeqWithLength {
    () => {
        pub trait CollectSeqWithLength : Serializer { fn collect_seq_with_length < I > (self , length : usize , iterable : I) -> Result < Self :: Ok , Self :: Error > where I : IntoIterator , I :: Item : Serialize , { let mut count = 0 ; let mut seq = self . serialize_seq (Some (length)) ? ; for element in iterable { seq . serialize_element (& element) ? ; count += 1 ; } debug_assert_eq ! (length , count , "collect_seq_with_length: length mismatch!") ; seq . end () } fn collect_seq_exact < I > (self , iterable : I) -> Result < Self :: Ok , Self :: Error > where I : IntoIterator , I :: Item : Serialize , I :: IntoIter : ExactSizeIterator , { let iter = iterable . into_iter () ; self . collect_seq_with_length (iter . len () , iter) } }
    };
}

CollectSeqWithLength!()