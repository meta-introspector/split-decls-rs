macro_rules! deps {
    () => {
        Current!();
    };
}

macro_rules! impl_1038 {
    () => {
        deps!();
        impl Current { fn new () -> Self { Self (task01 :: current ()) } fn as_waker (& self) -> WakerRef < '_ > { unsafe fn ptr_to_current < 'a > (ptr : * const ()) -> & 'a Current { unsafe { & * (ptr as * const Current) } } fn current_to_ptr (current : & Current) -> * const () { current as * const Current as * const () } unsafe fn clone (ptr : * const ()) -> RawWaker { unsafe { mem :: transmute :: < task03 :: Waker , RawWaker > (task03 :: waker (Arc :: new (ptr_to_current (ptr) . clone () ,))) } } unsafe fn drop (_ : * const ()) { } unsafe fn wake (ptr : * const ()) { unsafe { ptr_to_current (ptr) . 0 . notify () } } let ptr = current_to_ptr (self) ; let vtable = & RawWakerVTable :: new (clone , wake , wake , drop) ; WakerRef :: new_unowned (std :: mem :: ManuallyDrop :: new (unsafe { task03 :: Waker :: from_raw (RawWaker :: new (ptr , vtable)) })) } }
    };
}

impl_1038!()