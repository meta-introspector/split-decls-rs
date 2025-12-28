macro_rules! Table {
    () => {
        # [doc = " A table of items."] # [doc = ""] # [doc = " Each item has a unique identifier."] # [doc = " Items can be deleted without changing the identifiers of other items."] # [derive (Debug)] pub struct Table < T > (Vec < T >) ;
    };
}

Table!()