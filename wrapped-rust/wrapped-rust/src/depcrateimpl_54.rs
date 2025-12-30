// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl RustWasm { fn new () -> RustWasm { RustWasm :: default () } fn interface < 'a > (& 'a mut self , identifier : Identifier < 'a > , wasm_import_module : & 'a str , resolve : & 'a Resolve , in_import : bool ,) -> InterfaceGenerator < 'a > { let mut sizes = SizeAlign :: default () ; sizes . fill (resolve) ; InterfaceGenerator { identifier , wasm_import_module , src : Source :: default () , in_import , r#gen : self , sizes , resolve , return_pointer_area_size : Default :: default () , return_pointer_area_align : Default :: default () , needs_runtime_module : false , } } fn emit_modules (& mut self , modules : Vec < (String , Vec < String >) >) { # [derive (Default)] struct Module { submodules : BTreeMap < String , Module > , contents : Vec < String > , } let mut map = Module :: default () ; for (module , path) in modules { let mut cur = & mut map ; for name in path [.. path . len () - 1] . iter () { cur = cur . submodules . entry (name . clone ()) . or_insert (Module :: default ()) ; } cur . contents . push (module) ; } emit (& mut self . src , map , & self . opts , true) ; fn emit (me : & mut Source , module : Module , opts : & Opts , toplevel : bool) { for (name , submodule) in module . submodules { if toplevel { if opts . format { uwriteln ! (me , "#[rustfmt::skip]") ; } uwriteln ! (me , "#[allow(dead_code, clippy::all)]") ; } uwriteln ! (me , "pub mod {name} {{") ; emit (me , submodule , opts , false) ; uwriteln ! (me , "}}") ; } for submodule in module . contents { uwriteln ! (me , "{submodule}") ; } } } fn runtime_path (& self) -> & str { self . opts . runtime_path . as_deref () . unwrap_or ("wit_bindgen::rt") } fn bitflags_path (& self) -> String { self . opts . bitflags_path . to_owned () . unwrap_or (format ! ("{}::bitflags" , self . runtime_path ())) } fn async_support_path (& self) -> String { format ! ("{}::async_support" , self . runtime_path ()) } fn name_interface (& mut self , resolve : & Resolve , id : InterfaceId , name : & WorldKey , is_export : bool ,) -> Result < bool > { let with_name = resolve . name_world_key (name) ; let remapping = if is_export { & TypeGeneration :: Generate } else { match self . with . get (& with_name) { Some (remapping) => remapping , None => bail ! (MissingWith (with_name)) , } } ; self . generated_types . insert (with_name) ; let entry = match remapping { TypeGeneration :: Remap (remapped_path) => { let name = format ! ("__with_name{}" , self . with_name_counter) ; self . with_name_counter += 1 ; uwriteln ! (self . src , "#[allow(unfulfilled_lint_expectations, unused_imports)]") ; uwriteln ! (self . src , "use {remapped_path} as {name};") ; InterfaceName { remapped : true , path : name , } } TypeGeneration :: Generate => { let path = compute_module_path (name , resolve , is_export) . join ("::") ; InterfaceName { remapped : false , path , } } } ; let remapped = entry . remapped ; self . interface_names . insert (id , entry) ; Ok (remapped) } fn finish_runtime_module (& mut self) { if ! self . rt_module . is_empty () { if self . opts . format { uwriteln ! (self . src , "#[rustfmt::skip]") ; } self . src . push_str ("mod _rt {\n") ; self . src . push_str ("#![allow(dead_code, clippy::all)]\n") ; let mut emitted = IndexSet :: new () ; while ! self . rt_module . is_empty () { for item in mem :: take (& mut self . rt_module) { if emitted . insert (item) { self . emit_runtime_item (item) ; } } } self . src . push_str ("}\n") ; } if ! self . future_payloads . is_empty () { let async_support = self . async_support_path () ; self . src . push_str (& format ! ("\
pub mod wit_future {{
    #![allow(dead_code, unused_variables, clippy::all)]

    #[doc(hidden)]
    pub trait FuturePayload: Unpin + Sized + 'static {{
        const VTABLE: &'static {async_support}::FutureVtable<Self>;
    }}")) ; for code in self . future_payloads . values () { self . src . push_str (code) ; } self . src . push_str (& format ! ("\
    /// Creates a new Component Model `future` with the specified payload type.
    ///
    /// The `default` function provided computes the default value to be sent in
    /// this future if no other value was otherwise sent.
    pub fn new<T: FuturePayload>(default: fn() -> T) -> ({async_support}::FutureWriter<T>, {async_support}::FutureReader<T>) {{
        unsafe {{ {async_support}::future_new::<T>(default, T::VTABLE) }}
    }}
}}
                " ,)) ; } if ! self . stream_payloads . is_empty () { let async_support = self . async_support_path () ; self . src . push_str (& format ! ("\
pub mod wit_stream {{
    #![allow(dead_code, unused_variables, clippy::all)]

    pub trait StreamPayload: Unpin + Sized + 'static {{
        const VTABLE: &'static {async_support}::StreamVtable<Self>;
    }}")) ; for code in self . stream_payloads . values () { self . src . push_str (code) ; } self . src . push_str (& format ! ("\
    /// Creates a new Component Model `stream` with the specified payload type.
    pub fn new<T: StreamPayload>() -> ({async_support}::StreamWriter<T>, {async_support}::StreamReader<T>) {{
        unsafe {{ {async_support}::stream_new::<T>(T::VTABLE) }}
    }}
}}
                ") ,) ; } } fn emit_runtime_item (& mut self , item : RuntimeItem) { match item { RuntimeItem :: AllocCrate => { uwriteln ! (self . src , "extern crate alloc as alloc_crate;") ; } RuntimeItem :: StdAllocModule => { self . rt_module . insert (RuntimeItem :: AllocCrate) ; uwriteln ! (self . src , "pub use alloc_crate::alloc;") ; } RuntimeItem :: StringType => { self . rt_module . insert (RuntimeItem :: AllocCrate) ; uwriteln ! (self . src , "pub use alloc_crate::string::String;") ; } RuntimeItem :: BoxType => { self . rt_module . insert (RuntimeItem :: AllocCrate) ; uwriteln ! (self . src , "pub use alloc_crate::boxed::Box;") ; } RuntimeItem :: VecType => { self . rt_module . insert (RuntimeItem :: AllocCrate) ; uwriteln ! (self . src , "pub use alloc_crate::vec::Vec;") ; } RuntimeItem :: CabiDealloc => { self . rt_module . insert (RuntimeItem :: StdAllocModule) ; self . src . push_str ("\
pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
    if size == 0 {
        return;
    }
    unsafe {
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
}
                    " ,) ; } RuntimeItem :: StringLift => { self . rt_module . insert (RuntimeItem :: StringType) ; self . src . push_str ("\
pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
    if cfg!(debug_assertions) {
        String::from_utf8(bytes).unwrap()
    } else {
        unsafe { String::from_utf8_unchecked(bytes) }
    }
}
                    " ,) ; } RuntimeItem :: InvalidEnumDiscriminant => { self . src . push_str ("\
pub unsafe fn invalid_enum_discriminant<T>() -> T {
    if cfg!(debug_assertions) {
        panic!(\"invalid enum discriminant\")
    } else {
        unsafe { core::hint::unreachable_unchecked() }
    }
}
                    " ,) ; } RuntimeItem :: CharLift => { self . src . push_str ("\
pub unsafe fn char_lift(val: u32) -> char {
    if cfg!(debug_assertions) {
        core::char::from_u32(val).unwrap()
    } else {
        unsafe { core::char::from_u32_unchecked(val) }
    }
}
                    " ,) ; } RuntimeItem :: BoolLift => { self . src . push_str ("\
pub unsafe fn bool_lift(val: u8) -> bool {
    if cfg!(debug_assertions) {
        match val {
            0 => false,
            1 => true,
            _ => panic!(\"invalid bool discriminant\"),
        }
    } else {
        val != 0
    }
}
                    " ,) ; } RuntimeItem :: RunCtorsOnce => { let rt = self . runtime_path () ; self . src . push_str (& format ! (r#"
#[cfg(target_arch = "wasm32")]
pub fn run_ctors_once() {{
    {rt}::run_ctors_once();
}}
                    "# ,)) ; } RuntimeItem :: AsI32 => { self . emit_runtime_as_trait ("i32" , & ["i32" , "u32" , "i16" , "u16" , "i8" , "u8" , "char" , "usize"] ,) ; } RuntimeItem :: AsI64 => { self . emit_runtime_as_trait ("i64" , & ["i64" , "u64"]) ; } RuntimeItem :: AsF32 => { self . emit_runtime_as_trait ("f32" , & ["f32"]) ; } RuntimeItem :: AsF64 => { self . emit_runtime_as_trait ("f64" , & ["f64"]) ; } RuntimeItem :: ResourceType => { self . src . push_str (r#"

use core::fmt;
use core::marker;
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};

/// A type which represents a component model resource, either imported or
/// exported into this component.
///
/// This is a low-level wrapper which handles the lifetime of the resource
/// (namely this has a destructor). The `T` provided defines the component model
/// intrinsics that this wrapper uses.
///
/// One of the chief purposes of this type is to provide `Deref` implementations
/// to access the underlying data when it is owned.
///
/// This type is primarily used in generated code for exported and imported
/// resources.
#[repr(transparent)]
pub struct Resource<T: WasmResource> {
    // NB: This would ideally be `u32` but it is not. The fact that this has
    // interior mutability is not exposed in the API of this type except for the
    // `take_handle` method which is supposed to in theory be private.
    //
    // This represents, almost all the time, a valid handle value. When it's
    // invalid it's stored as `u32::MAX`.
    handle: AtomicU32,
    _marker: marker::PhantomData<T>,
}

/// A trait which all wasm resources implement, namely providing the ability to
/// drop a resource.
///
/// This generally is implemented by generated code, not user-facing code.
#[allow(clippy::missing_safety_doc)]
pub unsafe trait WasmResource {
    /// Invokes the `[resource-drop]...` intrinsic.
    unsafe fn drop(handle: u32);
}

impl<T: WasmResource> Resource<T> {
    #[doc(hidden)]
    pub unsafe fn from_handle(handle: u32) -> Self {
        debug_assert!(handle != 0 && handle != u32::MAX);
        Self {
            handle: AtomicU32::new(handle),
            _marker: marker::PhantomData,
        }
    }

    /// Takes ownership of the handle owned by `resource`.
    ///
    /// Note that this ideally would be `into_handle` taking `Resource<T>` by
    /// ownership. The code generator does not enable that in all situations,
    /// unfortunately, so this is provided instead.
    ///
    /// Also note that `take_handle` is in theory only ever called on values
    /// owned by a generated function. For example a generated function might
    /// take `Resource<T>` as an argument but then call `take_handle` on a
    /// reference to that argument. In that sense the dynamic nature of
    /// `take_handle` should only be exposed internally to generated code, not
    /// to user code.
    #[doc(hidden)]
    pub fn take_handle(resource: &Resource<T>) -> u32 {
        resource.handle.swap(u32::MAX, Relaxed)
    }

    #[doc(hidden)]
    pub fn handle(resource: &Resource<T>) -> u32 {
        resource.handle.load(Relaxed)
    }
}

impl<T: WasmResource> fmt::Debug for Resource<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Resource")
            .field("handle", &self.handle)
            .finish()
    }
}

impl<T: WasmResource> Drop for Resource<T> {
    fn drop(&mut self) {
        unsafe {
            match self.handle.load(Relaxed) {
                // If this handle was "taken" then don't do anything in the
                // destructor.
                u32::MAX => {}

                // ... but otherwise do actually destroy it with the imported
                // component model intrinsic as defined through `T`.
                other => T::drop(other),
            }
        }
    }
}
                    "# ,) ; } } } fn emit_runtime_as_trait (& mut self , ty : & str , to_convert : & [& str]) { let upcase = ty . to_uppercase () ; self . src . push_str (& format ! (r#"
pub fn as_{ty}<T: As{upcase}>(t: T) -> {ty} {{
    t.as_{ty}()
}}

pub trait As{upcase} {{
    fn as_{ty}(self) -> {ty};
}}

impl<'a, T: Copy + As{upcase}> As{upcase} for &'a T {{
    fn as_{ty}(self) -> {ty} {{
        (*self).as_{ty}()
    }}
}}
            "#)) ; for to_convert in to_convert { self . src . push_str (& format ! (r#"
impl As{upcase} for {to_convert} {{
    #[inline]
    fn as_{ty}(self) -> {ty} {{
        self as {ty}
    }}
}}
                "#)) ; } } # [doc = " Generates an `export!` macro for the `world_id` specified."] # [doc = ""] # [doc = " This will generate a macro which will then itself invoke all the"] # [doc = " other macros collected in `self.export_macros` prior. All these macros"] # [doc = " are woven together in this single invocation."] fn finish_export_macro (& mut self , resolve : & Resolve , world_id : WorldId) { if self . export_macros . is_empty () { return ; } let world = & resolve . worlds [world_id] ; let world_name = world . name . to_snake_case () ; let default_bindings_module = self . opts . default_bindings_module . clone () . unwrap_or ("self" . to_string ()) ; let (macro_export , use_vis) = if self . opts . pub_export_macro { ("#[macro_export]" , "pub") } else { ("" , "pub(crate)") } ; let export_macro_name = self . opts . export_macro_name . as_deref () . unwrap_or ("export") . to_string () ; uwriteln ! (self . src , r#"
/// Generates `#[unsafe(no_mangle)]` functions to export the specified type as
/// the root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! {export_macro_name} {{ ($($t:tt)*) => (); }}
/// # trait Guest {{}}
/// struct MyType;
///
/// impl Guest for MyType {{
///     // ...
/// }}
///
/// {export_macro_name}!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]
{macro_export}
macro_rules! __export_{world_name}_impl {{
    ($ty:ident) => ({default_bindings_module}::{export_macro_name}!($ty with_types_in {default_bindings_module}););
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => ("#) ; for (name , path_to_types) in self . export_macros . iter () { let mut path = "$($path_to_types_root)*" . to_string () ; if ! path_to_types . is_empty () { path . push_str ("::") ; path . push_str (path_to_types) } uwriteln ! (self . src , "{path}::{name}!($ty with_types_in {path});") ; } if self . opts . pub_export_macro { uwriteln ! (self . src , "const _: () = {{") ; self . emit_custom_section (resolve , world_id , "imports and exports" , None) ; uwriteln ! (self . src , "}};") ; } uwriteln ! (self . src , ")\n}}") ; uwriteln ! (self . src , "#[doc(inline)]\n\
            {use_vis} use __export_{world_name}_impl as {export_macro_name};") ; if self . opts . stubs { uwriteln ! (self . src , "export!(Stub);") ; } } # [doc = " Generates a `#[link_section]` custom section to get smuggled through"] # [doc = " `wasm-ld`."] # [doc = ""] # [doc = " This custom section is an encoding of the component metadata and will be"] # [doc = " used as part of the `wit-component`-based componentization process."] # [doc = ""] # [doc = " The `section_suffix` here is used to distinguish the multiple sections"] # [doc = " that this generator emits, and `func_name` is an optional function to"] # [doc = " generate next to this which is used to force rustc to at least visit"] # [doc = " this `static` and codegen it."] fn emit_custom_section (& mut self , resolve : & Resolve , world_id : WorldId , section_suffix : & str , func_name : Option < & str > ,) { if self . opts . format { uwriteln ! (self . src , "#[rustfmt::skip]") ; } self . src . push_str ("\n#[cfg(target_arch = \"wasm32\")]\n") ; let opts_suffix = self . opts . type_section_suffix . as_deref () . unwrap_or ("") ; let world = & resolve . worlds [world_id] ; let world_name = & world . name ; let pkg = & resolve . packages [world . package . unwrap ()] . name ; let version = env ! ("CARGO_PKG_VERSION") ; self . src . push_str (& format ! ("#[unsafe(link_section = \"component-type:wit-bindgen:{version}:\
             {pkg}:{world_name}:{section_suffix}{opts_suffix}\")]\n")) ; let mut producers = wasm_metadata :: Producers :: empty () ; producers . add ("processed-by" , env ! ("CARGO_PKG_NAME") , env ! ("CARGO_PKG_VERSION") ,) ; let component_type = wit_component :: metadata :: encode (resolve , world_id , wit_component :: StringEncoding :: UTF8 , Some (& producers) ,) . unwrap () ; self . src . push_str ("#[doc(hidden)]\n") ; self . src . push_str ("#[allow(clippy::octal_escapes)]\n") ; self . src . push_str (& format ! ("pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; {}] = *b\"\\\n" , component_type . len ())) ; let old_indent = self . src . set_indent (0) ; let mut line_length = 0 ; let s = self . src . as_mut_string () ; for byte in component_type . iter () { if line_length >= 80 { s . push_str ("\\\n") ; line_length = 0 ; } match byte { b'\\' => { s . push_str ("\\\\") ; line_length += 2 ; } b'"' => { s . push_str ("\\\"") ; line_length += 2 ; } b if b . is_ascii_alphanumeric () || b . is_ascii_punctuation () => { s . push (char :: from (* byte)) ; line_length += 1 ; } 0 => { s . push_str ("\\0") ; line_length += 2 ; } _ => { uwrite ! (s , "\\x{:02x}" , byte) ; line_length += 4 ; } } } self . src . push_str ("\";\n") ; self . src . set_indent (old_indent) ; if let Some (func_name) = func_name { let rt = self . runtime_path () . to_string () ; uwriteln ! (self . src , "
                #[inline(never)]
                #[doc(hidden)]
                pub fn {func_name}() {{
                    {rt}::maybe_link_cabi_realloc();
                }}
            " ,) ; } } fn is_async (& mut self , resolve : & Resolve , interface : Option < & WorldKey > , func : & Function , is_import : bool ,) -> bool { self . opts . async_ . is_async (resolve , interface , func , is_import) } }
};
}
