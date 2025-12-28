macro_rules! deps {
    () => {
        Box!();
    };
}

macro_rules! unsize_box {
    () => {
        deps!();
        # [doc = " Allows turning a [`Box<T: Sized, A>`][boxed::Box] into a [`Box<U: ?Sized, A>`][boxed::Box] where `T` can be unsizing-coerced into a `U`."] # [doc = ""] # [doc = " This is the only way to create an `allocator_api2::boxed::Box` of an unsized type on stable."] # [doc = ""] # [doc = " With the standard library's `alloc::boxed::Box`, this is done automatically using the unstable unsize traits, but this crate's Box"] # [doc = " can't take advantage of that machinery on stable. So, we need to use type inference and the fact that you *can*"] # [doc = " still coerce the inner pointer of a box to get the compiler to help us unsize it using this macro."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use allocator_api2::unsize_box;"] # [doc = " use allocator_api2::boxed::Box;"] # [doc = " use core::any::Any;"] # [doc = ""] # [doc = " let sized_box: Box<u64> = Box::new(0);"] # [doc = " let unsized_box: Box<dyn Any> = unsize_box!(sized_box);"] # [doc = " ```"] # [macro_export] # [cfg (feature = "alloc")] macro_rules ! unsize_box { ($ boxed : expr $ (,) ?) => ({ let (ptr , allocator) = $ crate :: boxed :: Box :: into_raw_with_allocator ($ boxed) ; let ptr : * mut _ = ptr ; unsafe { $ crate :: boxed :: Box :: from_raw_in (ptr , allocator) } }) }
    };
}

unsize_box!()