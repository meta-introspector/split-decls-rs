macro_rules! DependencyCollector {
    () => {
        pub struct DependencyCollector { pub dependencies : HashSet < String > , }
    };
}

DependencyCollector!()