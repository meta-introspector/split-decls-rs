macro_rules! Node {
    () => {
        struct Node < T > { next : AtomicPtr < Self > , value : Option < T > , }
    };
}

Node!()