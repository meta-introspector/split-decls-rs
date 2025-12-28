macro_rules! Child {
    () => {
        # [derive (Debug)] struct Child < T > { id : T , children : Vec < usize > , }
    };
}

Child!();