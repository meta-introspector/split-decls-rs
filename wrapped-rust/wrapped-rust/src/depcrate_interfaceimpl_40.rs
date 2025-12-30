// Generated macro for impl_40 (impl)
macro_rules! Depcrate_interfaceimpl_40 {
() => {
// Module: crate::interface
// Provides: {"impl_40"}
// Dependencies: {}
impl < 'a > wit_bindgen_core :: InterfaceGenerator < 'a > for InterfaceGenerator < 'a > { fn resolve (& self) -> & 'a Resolve { self . resolve } fn type_record (& mut self , id : TypeId , _name : & str , record : & Record , docs : & Docs) { self . print_typedef_record (id , record , docs) ; } fn type_resource (& mut self , _id : TypeId , name : & str , docs : & Docs) { self . rustdoc (docs) ; let camel = to_upper_camel_case (name) ; let resource = self . path_to_resource () ; let wasm_import_module = if self . in_import { uwriteln ! (self . src , r#"
                    #[derive(Debug)]
                    #[repr(transparent)]
                    pub struct {camel} {{
                        handle: {resource}<{camel}>,
                    }}

                    impl {camel} {{
                        #[doc(hidden)]
                        pub unsafe fn from_handle(handle: u32) -> Self {{
                            Self {{
                                handle: unsafe {{ {resource}::from_handle(handle) }},
                            }}
                        }}

                        #[doc(hidden)]
                        pub fn take_handle(&self) -> u32 {{
                            {resource}::take_handle(&self.handle)
                        }}

                        #[doc(hidden)]
                        pub fn handle(&self) -> u32 {{
                            {resource}::handle(&self.handle)
                        }}
                    }}
                "#) ; self . wasm_import_module . to_string () } else { let module = match self . identifier { Identifier :: Interface (_ , key) => self . resolve . name_world_key (key) , Identifier :: World (_) => unimplemented ! ("resource exports from worlds") , Identifier :: StreamOrFuturePayload => unreachable ! () , } ; let box_path = self . path_to_box () ; uwriteln ! (self . src , r#"
#[derive(Debug)]
#[repr(transparent)]
pub struct {camel} {{
    handle: {resource}<{camel}>,
}}

type _{camel}Rep<T> = Option<T>;

impl {camel} {{
    /// Creates a new resource from the specified representation.
    ///
    /// This function will create a new resource handle by moving `val` onto
    /// the heap and then passing that heap pointer to the component model to
    /// create a handle. The owned handle is then returned as `{camel}`.
    pub fn new<T: Guest{camel}>(val: T) -> Self {{
        Self::type_guard::<T>();
        let val: _{camel}Rep<T> = Some(val);
        let ptr: *mut _{camel}Rep<T> =
            {box_path}::into_raw({box_path}::new(val));
        unsafe {{
            Self::from_handle(T::_resource_new(ptr.cast()))
        }}
    }}

    /// Gets access to the underlying `T` which represents this resource.
    pub fn get<T: Guest{camel}>(&self) -> &T {{
        let ptr = unsafe {{ &*self.as_ptr::<T>() }};
        ptr.as_ref().unwrap()
    }}

    /// Gets mutable access to the underlying `T` which represents this
    /// resource.
    pub fn get_mut<T: Guest{camel}>(&mut self) -> &mut T {{
        let ptr = unsafe {{ &mut *self.as_ptr::<T>() }};
        ptr.as_mut().unwrap()
    }}

    /// Consumes this resource and returns the underlying `T`.
    pub fn into_inner<T: Guest{camel}>(self) -> T {{
        let ptr = unsafe {{ &mut *self.as_ptr::<T>() }};
        ptr.take().unwrap()
    }}

    #[doc(hidden)]
    pub unsafe fn from_handle(handle: u32) -> Self {{
        Self {{
            handle: unsafe {{ {resource}::from_handle(handle) }},
        }}
    }}

    #[doc(hidden)]
    pub fn take_handle(&self) -> u32 {{
        {resource}::take_handle(&self.handle)
    }}

    #[doc(hidden)]
    pub fn handle(&self) -> u32 {{
        {resource}::handle(&self.handle)
    }}

    // It's theoretically possible to implement the `Guest{camel}` trait twice
    // so guard against using it with two different types here.
    #[doc(hidden)]
    fn type_guard<T: 'static>() {{
        use core::any::TypeId;
        static mut LAST_TYPE: Option<TypeId> = None;
        unsafe {{
            assert!(!cfg!(target_feature = "atomics"));
            let id = TypeId::of::<T>();
            match LAST_TYPE {{
                Some(ty) => assert!(ty == id, "cannot use two types with this resource type"),
                None => LAST_TYPE = Some(id),
            }}
        }}
    }}

    #[doc(hidden)]
    pub unsafe fn dtor<T: 'static>(handle: *mut u8) {{
        Self::type_guard::<T>();
        let _ = unsafe {{ {box_path}::from_raw(handle as *mut _{camel}Rep<T>) }};
    }}

    fn as_ptr<T: Guest{camel}>(&self) -> *mut _{camel}Rep<T> {{
       {camel}::type_guard::<T>();
       T::_resource_rep(self.handle()).cast()
    }}
}}

/// A borrowed version of [`{camel}`] which represents a borrowed value
/// with the lifetime `'a`.
#[derive(Debug)]
#[repr(transparent)]
pub struct {camel}Borrow<'a> {{
    rep: *mut u8,
    _marker: core::marker::PhantomData<&'a {camel}>,
}}

impl<'a> {camel}Borrow<'a>{{
    #[doc(hidden)]
    pub unsafe fn lift(rep: usize) -> Self {{
        Self {{
            rep: rep as *mut u8,
            _marker: core::marker::PhantomData,
        }}
    }}

    /// Gets access to the underlying `T` in this resource.
    pub fn get<T: Guest{camel}>(&self) -> &'a T {{
       let ptr = unsafe {{ &mut *self.as_ptr::<T>() }};
       ptr.as_ref().unwrap()
    }}

    // NB: mutable access is not allowed due to the component model allowing
    // multiple borrows of the same resource.

    fn as_ptr<T: 'static>(&self) -> *mut _{camel}Rep<T> {{
       {camel}::type_guard::<T>();
       self.rep.cast()
    }}
}}
                "#) ; format ! ("[export]{module}") } ; let wasm_resource = self . path_to_wasm_resource () ; let intrinsic = crate :: declare_import (& wasm_import_module , & format ! ("[resource-drop]{name}") , "drop" , & [abi :: WasmType :: I32] , & [] ,) ; uwriteln ! (self . src , r#"
                unsafe impl {wasm_resource} for {camel} {{
                     #[inline]
                     unsafe fn drop(_handle: u32) {{
                         {intrinsic}
                         unsafe {{ drop(_handle as i32); }}
                     }}
                }}
            "#) ; } fn type_tuple (& mut self , id : TypeId , _name : & str , tuple : & Tuple , docs : & Docs) { for (name , mode) in self . modes_of (id) { self . rustdoc (docs) ; self . push_str (& format ! ("pub type {}" , name)) ; self . print_generics (mode . lifetime) ; self . push_str (" = (") ; for ty in tuple . types . iter () { let mode = self . filter_mode (ty , mode) ; self . print_ty (ty , mode) ; self . push_str (",") ; } self . push_str (");\n") ; } } fn type_flags (& mut self , _id : TypeId , name : & str , flags : & Flags , docs : & Docs) { self . src . push_str (& format ! ("{bitflags}::bitflags! {{\n" , bitflags = self . r#gen . bitflags_path ())) ; self . rustdoc (docs) ; let repr = RustFlagsRepr :: new (flags) ; self . src . push_str (& format ! ("#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone, Copy)]\npub struct {}: {repr} {{\n" , name . to_upper_camel_case () ,)) ; for (i , flag) in flags . flags . iter () . enumerate () { self . rustdoc (& flag . docs) ; self . src . push_str (& format ! ("const {} = 1 << {};\n" , flag . name . to_shouty_snake_case () , i ,)) ; } self . src . push_str ("}\n") ; self . src . push_str ("}\n") ; } fn type_variant (& mut self , id : TypeId , _name : & str , variant : & Variant , docs : & Docs) { self . print_typedef_variant (id , variant , docs) ; } fn type_option (& mut self , id : TypeId , _name : & str , payload : & Type , docs : & Docs) { self . print_typedef_option (id , payload , docs) ; } fn type_result (& mut self , id : TypeId , _name : & str , result : & Result_ , docs : & Docs) { self . print_typedef_result (id , result , docs) ; } fn type_enum (& mut self , id : TypeId , name : & str , enum_ : & Enum , docs : & Docs) { self . print_typedef_enum (id , name , enum_ , docs , & [] , Box :: new (| _ | String :: new ())) ; let name = to_upper_camel_case (name) ; let mut cases = String :: new () ; let repr = int_repr (enum_ . tag ()) ; for (i , case) in enum_ . cases . iter () . enumerate () { let case = case . name . to_upper_camel_case () ; cases . push_str (& format ! ("{i} => {name}::{case},\n")) ; } uwriteln ! (self . src , r#"
                impl {name} {{
                    #[doc(hidden)]
                    pub unsafe fn _lift(val: {repr}) -> {name} {{
                        if !cfg!(debug_assertions) {{
                            return unsafe {{ ::core::mem::transmute(val) }};
                        }}

                        match val {{
                            {cases}
                            _ => panic!("invalid enum discriminant"),
                        }}
                    }}
                }}
            "#) ; } fn type_alias (& mut self , id : TypeId , _name : & str , ty : & Type , docs : & Docs) { self . print_typedef_alias (id , ty , docs) ; } fn type_list (& mut self , id : TypeId , _name : & str , ty : & Type , docs : & Docs) { for (name , mode) in self . modes_of (id) { self . rustdoc (docs) ; self . push_str (& format ! ("pub type {}" , name)) ; self . print_generics (mode . lifetime) ; self . push_str (" = ") ; self . print_list (ty , mode) ; self . push_str (";\n") ; } } fn type_future (& mut self , _id : TypeId , name : & str , ty : & Option < Type > , docs : & Docs) { let async_support = self . r#gen . async_support_path () ; let mode = TypeMode { style : TypeOwnershipStyle :: Owned , lists_borrowed : false , lifetime : None , } ; self . rustdoc (docs) ; self . push_str (& format ! ("pub type {}" , name . to_upper_camel_case ())) ; self . print_generics (mode . lifetime) ; self . push_str (" = ") ; self . push_str (& format ! ("{async_support}::FutureReader<")) ; self . print_optional_ty (ty . as_ref () , mode) ; self . push_str (">") ; self . push_str (";\n") ; } fn type_stream (& mut self , _id : TypeId , name : & str , ty : & Option < Type > , docs : & Docs) { let async_support = self . r#gen . async_support_path () ; let mode = TypeMode { style : TypeOwnershipStyle :: Owned , lists_borrowed : false , lifetime : None , } ; self . rustdoc (docs) ; self . push_str (& format ! ("pub type {}" , name . to_upper_camel_case ())) ; self . print_generics (mode . lifetime) ; self . push_str (" = ") ; self . push_str (& format ! ("{async_support}::StreamReader<")) ; self . print_optional_ty (ty . as_ref () , mode) ; self . push_str (">") ; self . push_str (";\n") ; } fn type_builtin (& mut self , _id : TypeId , name : & str , ty : & Type , docs : & Docs) { self . rustdoc (docs) ; self . src . push_str (& format ! ("pub type {}" , name . to_upper_camel_case ())) ; self . src . push_str (" = ") ; self . print_ty (ty , TypeMode :: owned ()) ; self . src . push_str (";\n") ; } }
};
}
