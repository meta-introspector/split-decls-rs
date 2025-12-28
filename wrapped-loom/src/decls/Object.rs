macro_rules! Object {
    () => {
        pub (super) trait Object : Sized { type Entry ; # [doc = " Convert an object into an entry"] fn into_entry (self) -> Self :: Entry ; # [doc = " Convert an entry ref into an object ref"] fn get_ref (entry : & Self :: Entry) -> Option < & Self > ; # [doc = " Convert a mutable entry ref into a mutable object ref"] fn get_mut (entry : & mut Self :: Entry) -> Option < & mut Self > ; }
    };
}

Object!()