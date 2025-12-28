macro_rules! deps {
    () => {
        DependencyCollector!();
    };
}

macro_rules! count_dependencies {
    () => {
        deps!();
        pub fn count_dependencies (item : & Item) -> usize { let mut collector = DependencyCollector :: new () ; collector . visit_item (item) ; collector . dependencies . len () }
    };
}

count_dependencies!()