macro_rules! IsTuple {
    () => {
        # [derive (Copy , Clone)] pub (crate) enum IsTuple { No , Yes , }
    };
}

IsTuple!()