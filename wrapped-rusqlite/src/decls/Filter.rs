macro_rules! Filter {
    () => {
        type Filter = Option < Box < dyn Fn (& str) -> bool > > ;
    };
}

Filter!();