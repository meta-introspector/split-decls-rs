mkuse!{use rustc_span :: { Symbol , sym } ;}
mkuse!{use crate :: attr :: { self , AttributeExt } ;}
mkitem!{mkenum!{# [derive (Debug)] pub enum EntryPointType { # [doc = " This function is not an entrypoint."] None , # [doc = " This is a function called `main` at the root level."] # [doc = " ```"] # [doc = " fn main() {}"] # [doc = " ```"] MainNamed , # [doc = " This is a function with the `#[rustc_main]` attribute."] # [doc = " Used by the testing harness to create the test entrypoint."] # [doc = " ```ignore (clashes with test entrypoint)"] # [doc = " #[rustc_main]"] # [doc = " fn main() {}"] # [doc = " ```"] RustcMainAttr , # [doc = " This function is **not** an entrypoint but simply named `main` (not at the root)."] # [doc = " This is only used for diagnostics."] # [doc = " ```"] # [doc = " #[allow(dead_code)]"] # [doc = " mod meow {"] # [doc = "     fn main() {}"] # [doc = " }"] # [doc = " ```"] OtherMain , }}}

macro_rules! entry_point_type_introspect {
    () => {
        emit_message!("📊 INTROSPECT: Function entry_point_type in module {}", module_path!());
    };
}

mkfn!{
    entry_point_type_introspect!();
    pub fn entry_point_type (attrs : & [impl AttributeExt] , at_root : bool , name : Option < Symbol > ,) -> EntryPointType { if attr :: contains_name (attrs , sym :: rustc_main) { EntryPointType :: RustcMainAttr } else if let Some (name) = name && name == sym :: main { if at_root { EntryPointType :: MainNamed } else { EntryPointType :: OtherMain } } else { EntryPointType :: None } }
}