macro_rules! deps {
    () => {
        MetaItemInner!();
    };
}

macro_rules! list_contains_name {
    () => {
        deps!();
        pub fn list_contains_name (items : & [MetaItemInner] , name : Symbol) -> bool { items . iter () . any (| item | item . has_name (name)) }
    };
}

list_contains_name!()