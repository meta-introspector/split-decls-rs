macro_rules! OwningIter {
    () => {
        pub struct OwningIter < K > { inner : crate :: iter :: OwningIter < K , () > , }
    };
}

OwningIter!();