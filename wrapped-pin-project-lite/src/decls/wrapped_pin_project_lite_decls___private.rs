use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[doc(hidden)]
#[allow(missing_debug_implementations)]
pub mod __private {
    use core::mem::ManuallyDrop;
    #[doc(hidden)]
    pub use core::{
        marker::{PhantomData, PhantomPinned, Unpin},
        ops::Drop,
        pin::Pin,
        ptr,
    };
    #[doc(hidden)]
    pub type PinnedFieldsOf<T> =
        <PinnedFieldsOfHelperStruct<T> as PinnedFieldsOfHelperTrait>::Actual;
    #[doc(hidden)]
    pub trait PinnedFieldsOfHelperTrait {
        type Actual: ?Sized;
    }
    #[doc(hidden)]
    pub struct PinnedFieldsOfHelperStruct<T: ?Sized>(T);
    impl<T: ?Sized> PinnedFieldsOfHelperTrait for PinnedFieldsOfHelperStruct<T> {
        type Actual = T;
    }
    #[doc(hidden)]
    pub struct AlwaysUnpin<T: ?Sized>(PhantomData<T>);
    impl<T: ?Sized> Unpin for AlwaysUnpin<T> {}
    #[doc(hidden)]
    pub struct UnsafeDropInPlaceGuard<T: ?Sized>(*mut T);
    impl<T: ?Sized> UnsafeDropInPlaceGuard<T> {
        #[doc(hidden)]
        pub unsafe fn new(ptr: *mut T) -> Self {
            Self(ptr)
        }
    }
    impl<T: ?Sized> Drop for UnsafeDropInPlaceGuard<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::drop_in_place(self.0);
            }
        }
    }
    #[doc(hidden)]
    pub struct UnsafeOverwriteGuard<T> {
        target: *mut T,
        value: ManuallyDrop<T>,
    }
    impl<T> UnsafeOverwriteGuard<T> {
        #[doc(hidden)]
        pub unsafe fn new(target: *mut T, value: T) -> Self {
            Self {
                target,
                value: ManuallyDrop::new(value),
            }
        }
    }
    impl<T> Drop for UnsafeOverwriteGuard<T> {
        fn drop(&mut self) {
            unsafe {
                ptr::write(self.target, ptr::read(&*self.value));
            }
        }
    }
}
