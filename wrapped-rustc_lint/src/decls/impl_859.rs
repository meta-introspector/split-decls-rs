macro_rules! deps {
    () => {
        EarlyContext!();
    };
}

macro_rules! impl_859 {
    () => {
        deps!();
        impl EarlyLintPass for UnusedImportBraces { fn check_item (& mut self , cx : & EarlyContext < '_ > , item : & ast :: Item) { if let ast :: ItemKind :: Use (ref use_tree) = item . kind { self . check_use_tree (cx , use_tree , item) ; } } }
    };
}

impl_859!()