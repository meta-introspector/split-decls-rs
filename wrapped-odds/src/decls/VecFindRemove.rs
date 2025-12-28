macro_rules! VecFindRemove {
    () => {
        pub trait VecFindRemove { type Item ; # [doc = " Linear search for the first element equal to `elt` and remove"] # [doc = " it if found."] # [doc = ""] # [doc = " Return its index and the value itself."] fn find_remove < U > (& mut self , elt : & U) -> Option < (usize , Self :: Item) > where Self :: Item : PartialEq < U > ; # [doc = " Linear search for the last element equal to `elt` and remove"] # [doc = " it if found."] # [doc = ""] # [doc = " Return its index and the value itself."] fn rfind_remove < U > (& mut self , elt : & U) -> Option < (usize , Self :: Item) > where Self :: Item : PartialEq < U > ; }
    };
}

VecFindRemove!();