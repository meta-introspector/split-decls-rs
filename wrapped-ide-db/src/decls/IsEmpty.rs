macro_rules! IsEmpty {
    () => {
        trait IsEmpty { fn is_empty (& self) -> bool ; }
    };
}

IsEmpty!();