macro_rules! GuardOwningIter {
    () => {
        type GuardOwningIter < K , V > = hash_table :: IntoIter < (K , V) > ;
    };
}

GuardOwningIter!()