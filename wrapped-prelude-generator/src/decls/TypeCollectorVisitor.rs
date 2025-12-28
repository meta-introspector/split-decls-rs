macro_rules! TypeCollectorVisitor {
    () => {
        # [derive (Debug , Default)] pub struct TypeCollectorVisitor { pub collected_types : HashSet < String > , }
    };
}

TypeCollectorVisitor!()