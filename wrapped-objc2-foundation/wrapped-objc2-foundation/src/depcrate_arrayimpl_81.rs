// Generated macro for impl_81 (impl)
macro_rules! Depcrate_arrayimpl_81 {
() => {
// Module: crate::array
// Provides: {"impl_81"}
// Dependencies: {}
# [doc = " Convenience mutation methods."] impl < ObjectType : Message > NSMutableArray < ObjectType > { # [doc = " Insert an object into the array at the given index."] # [doc = ""] # [doc = " # Panics"] # [doc = ""] # [doc = " Panics if the index is out of bounds."] # [doc (alias = "insertObject:atIndex:")] pub fn insert (& self , index : usize , obj : & ObjectType) { let len = self . len () ; if index <= len { self . insertObject_atIndex (obj , index) } else { panic ! ("insertion index (is {}) should be <= len (is {})" , index , len) ; } } # [doc = " Sort the array by the given comparison closure."] # [cfg (feature = "NSObjCRuntime")] # [doc (alias = "sortUsingFunction:context:")] pub fn sort_by < F : FnMut (& ObjectType , & ObjectType) -> core :: cmp :: Ordering > (& self , compare : F) { unsafe extern "C-unwind" fn compare_with_closure < ObjectType , F : FnMut (& ObjectType , & ObjectType) -> core :: cmp :: Ordering , > (obj1 : core :: ptr :: NonNull < ObjectType > , obj2 : core :: ptr :: NonNull < ObjectType > , context : * mut core :: ffi :: c_void ,) -> isize { let context : * mut F = context . cast () ; let closure : & mut F = unsafe { context . as_mut () . unwrap_unchecked () } ; let (obj1 , obj2) = unsafe { (obj1 . as_ref () , obj2 . as_ref ()) } ; crate :: NSComparisonResult :: from ((* closure) (obj1 , obj2)) as _ } let f : unsafe extern "C-unwind" fn (_ , _ , _) -> _ = compare_with_closure :: < ObjectType , F > ; let mut closure = compare ; let context : * mut F = & mut closure ; unsafe { self . sortUsingFunction_context (f , context . cast ()) } ; drop (closure) ; } }
};
}
