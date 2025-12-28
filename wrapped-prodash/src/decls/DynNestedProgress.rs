macro_rules! deps {
    () => {
        BoxedDynNestedProgress!();
        NestedProgress!();
        Progress!();
        Id!();
    };
}

macro_rules! DynNestedProgress {
    () => {
        deps!();
        # [doc = " An object-safe trait for describing hierarchical progress."] # [doc = ""] # [doc = " This will be automatically implemented for any type that implements"] # [doc = " [`NestedProgress`]."] pub trait DynNestedProgress : Progress + impls :: Sealed { # [doc = " See [`NestedProgress::add_child`]"] fn add_child (& mut self , name : String) -> BoxedDynNestedProgress ; # [doc = " See [`NestedProgress::add_child_with_id`]"] fn add_child_with_id (& mut self , name : String , id : Id) -> BoxedDynNestedProgress ; }
    };
}

DynNestedProgress!();