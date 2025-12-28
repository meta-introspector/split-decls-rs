macro_rules! deps {
    () => {
        UsageCache!();
    };
}

macro_rules! impl_91 {
    () => {
        deps!();
        impl UsageCache { fn find (& mut self , definition : & Definition) -> Option < & UsageSearchResult > { for (d , refs) in & self . usages { if d == definition { return Some (refs) ; } } None } }
    };
}

impl_91!();