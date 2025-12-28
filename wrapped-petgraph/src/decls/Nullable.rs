macro_rules! Nullable {
    () => {
        # [doc = " Wrapper trait for an `Option`, allowing user-defined structs to be input as containers when"] # [doc = " defining a null element."] # [doc = ""] # [doc = " Note: this trait is currently *sealed* and cannot be implemented for types outside this crate."] pub trait Nullable : Default + Into < Option < < Self as Nullable > :: Wrapped > > + private :: Sealed { # [doc (hidden)] type Wrapped ; # [doc (hidden)] fn new (value : Self :: Wrapped) -> Self ; # [doc (hidden)] fn as_ref (& self) -> Option < & Self :: Wrapped > ; # [doc (hidden)] fn as_mut (& mut self) -> Option < & mut Self :: Wrapped > ; # [doc (hidden)] fn is_null (& self) -> bool { self . as_ref () . is_none () } }
    };
}

Nullable!()