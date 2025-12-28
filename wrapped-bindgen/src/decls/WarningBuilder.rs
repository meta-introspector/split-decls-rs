macro_rules! WarningBuilder {
    () => {
        # [derive (Default)] pub (crate) struct WarningBuilder (RwLock < Vec < String > >) ;
    };
}

WarningBuilder!()