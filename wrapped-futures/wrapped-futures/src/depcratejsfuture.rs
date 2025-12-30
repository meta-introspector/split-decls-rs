// Generated macro for JsFuture (struct)
macro_rules! DepcrateJsFuture {
() => {
// Module: crate
// Provides: {"JsFuture"}
// Dependencies: {}
# [doc = " A Rust `Future` backed by a JavaScript `Promise`."] # [doc = ""] # [doc = " This type is constructed with a JavaScript `Promise` object and translates"] # [doc = " it to a Rust `Future`. This type implements the `Future` trait from the"] # [doc = " `futures` crate and will either succeed or fail depending on what happens"] # [doc = " with the JavaScript `Promise`."] # [doc = ""] # [doc = " Currently this type is constructed with `JsFuture::from`."] pub struct JsFuture { inner : Rc < RefCell < Inner > > , }
};
}
