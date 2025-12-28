macro_rules! BoxIter {
    () => {
        trait BoxIter { type Item ; fn last (self) -> Option < Self :: Item > ; }
    };
}

BoxIter!();