macro_rules! ListFolder {
    () => {
        struct ListFolder < T > { list : LinkedList < T > , }
    };
}

ListFolder!();