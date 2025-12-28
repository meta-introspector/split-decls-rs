macro_rules! potentially_plural_count {
    () => {
        pub fn potentially_plural_count (count : usize , word : & str) -> String { format ! ("{} {}{}" , count , word , pluralize ! (count)) }
    };
}

potentially_plural_count!()