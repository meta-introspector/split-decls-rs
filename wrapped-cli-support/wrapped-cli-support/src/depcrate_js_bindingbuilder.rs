// Generated macro for Builder (struct)
macro_rules! Depcrate_js_bindingBuilder {
() => {
// Module: crate::js::binding
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " A one-size-fits-all builder for processing WebIDL bindings and generating"] # [doc = " JS."] pub struct Builder < 'a , 'b > { # [doc = " Parent context used to expose helper functions and such."] pub cx : & 'a mut Context < 'b > , # [doc = " Whether or not this is building a constructor for a Rust class, and if"] # [doc = " so what class it's constructing."] constructor : Option < String > , # [doc = " Whether or not this is building a method of a Rust class instance, and"] # [doc = " whether or not the method consumes `self` or not."] method : Option < bool > , # [doc = " Whether this is a classless this function (receives JS `this` as first param)"] classless_this : bool , # [doc = " Whether or not we're catching exceptions from the main function"] # [doc = " invocation. Currently only used for imports."] catch : bool , # [doc = " Whether or not we're logging the error coming out of this intrinsic"] log_error : bool , }
};
}
