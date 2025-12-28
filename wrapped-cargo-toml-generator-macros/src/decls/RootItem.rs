macro_rules! deps {
    () => {
        KeyValue!();
    };
}

macro_rules! RootItem {
    () => {
        deps!();
        # [derive (Debug)] pub enum RootItem { Section (Ident , proc_macro2 :: TokenStream) , KeyValue (KeyValue) , }
    };
}

RootItem!();