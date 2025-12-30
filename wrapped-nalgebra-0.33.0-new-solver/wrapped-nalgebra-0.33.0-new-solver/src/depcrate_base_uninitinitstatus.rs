// Generated macro for InitStatus (trait)
macro_rules! Depcrate_base_uninitInitStatus {
() => {
// Module: crate::base::uninit
// Provides: {"InitStatus"}
// Dependencies: {}
# [doc = " This trait is used to write code that may work on matrices that may or may not"] # [doc = " be initialized."] # [doc = ""] # [doc = " This trait is used to describe how a value must be accessed to initialize it or"] # [doc = " to retrieve a reference or mutable reference. Typically, a function accepting"] # [doc = " both initialized and uninitialized inputs should have a `Status: InitStatus<T>`"] # [doc = " type parameter. Then the methods of the `Status` can be used to access the element."] # [doc = ""] # [doc = " # Safety"] # [doc = " This trait must not be implemented outside of this crate."] pub unsafe trait InitStatus < T > : Copy { # [doc = " The type of the values with the initialization status described by `Self`."] type Value ; # [doc = " Initialize the given element."] fn init (out : & mut Self :: Value , t : T) ; # [doc = " Retrieve a reference to the element, assuming that it is initialized."] # [doc = ""] # [doc = " # Safety"] # [doc = " This is unsound if the referenced value isn’t initialized."] unsafe fn assume_init_ref (t : & Self :: Value) -> & T ; # [doc = " Retrieve a mutable reference to the element, assuming that it is initialized."] # [doc = ""] # [doc = " # Safety"] # [doc = " This is unsound if the referenced value isn’t initialized."] unsafe fn assume_init_mut (t : & mut Self :: Value) -> & mut T ; }
};
}
