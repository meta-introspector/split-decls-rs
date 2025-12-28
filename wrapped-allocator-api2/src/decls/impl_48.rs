macro_rules! deps {
    () => {
        Allocator!();
        Box!();
    };
}

macro_rules! impl_48 {
    () => {
        deps!();
        impl < A : Allocator > Box < dyn Any + Send + Sync , A > { # [doc = " Attempt to downcast the box to a concrete type."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::any::Any;"] # [doc = ""] # [doc = " use allocator_api2::{boxed::Box, unsize_box};"] # [doc = ""] # [doc = " fn print_if_string(value: Box<dyn Any + Send + Sync>) {"] # [doc = "     if let Ok(string) = value.downcast::<String>() {"] # [doc = "         println!(\"String ({}): {}\", string.len(), string);"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " let my_string = \"Hello World\".to_string();"] # [doc = " print_if_string(unsize_box!(Box::new(my_string)));"] # [doc = " print_if_string(unsize_box!(Box::new(0i8)));"] # [doc = " ```"] # [inline (always)] pub fn downcast < T : Any > (self) -> Result < Box < T , A > , Self > { if self . is :: < T > () { unsafe { Ok (self . downcast_unchecked :: < T > ()) } } else { Err (self) } } # [doc = " Downcasts the box to a concrete type."] # [doc = ""] # [doc = " For a safe alternative see [`downcast`]."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use std::any::Any;"] # [doc = ""] # [doc = " use allocator_api2::{boxed::Box, unsize_box};"] # [doc = ""] # [doc = " let x: Box<dyn Any + Send + Sync> = unsize_box!(Box::new(1_usize));"] # [doc = ""] # [doc = " unsafe {"] # [doc = "     assert_eq!(*x.downcast_unchecked::<usize>(), 1);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The contained value must be of type `T`. Calling this method"] # [doc = " with the incorrect type is *undefined behavior*."] # [doc = ""] # [doc = " [`downcast`]: Self::downcast"] # [inline (always)] pub unsafe fn downcast_unchecked < T : Any > (self) -> Box < T , A > { debug_assert ! (self . is ::< T > ()) ; unsafe { let (raw , alloc) : (* mut (dyn Any + Send + Sync) , _) = Box :: into_raw_with_allocator (self) ; Box :: from_raw_in (raw as * mut T , alloc) } } }
    };
}

impl_48!();