macro_rules! IntoIter {
    () => {
        pub struct IntoIter < K > { iter : linked_hash_map :: IntoIter < K , () > , }
    };
}

IntoIter!();