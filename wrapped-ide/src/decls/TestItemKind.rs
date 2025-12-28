macro_rules! TestItemKind {
    () => {
        # [derive (Debug)] pub enum TestItemKind { Crate (base_db :: Crate) , Module , Function , }
    };
}

TestItemKind!();