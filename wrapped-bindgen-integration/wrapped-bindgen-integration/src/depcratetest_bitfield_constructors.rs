// Generated macro for test_bitfield_constructors (function)
macro_rules! Depcratetest_bitfield_constructors {
() => {
// Module: crate
// Provides: {"test_bitfield_constructors"}
// Dependencies: {}
# [test] fn test_bitfield_constructors () { use std :: mem ; let mut first = bindings :: bitfields :: First { _bitfield_1 : bindings :: bitfields :: First :: new_bitfield_1 (1 , 2 , 3) , } ; assert ! (unsafe { first . assert (1 , 2 , 3) }) ; let mut second = bindings :: bitfields :: Second { _bindgen_align : [] , _bitfield_1 : bindings :: bitfields :: Second :: new_bitfield_1 (1337 , true) , } ; assert ! (unsafe { second . assert (1337 , true) }) ; let mut third = bindings :: bitfields :: Third { _bindgen_align : [] , _bitfield_1 : bindings :: bitfields :: Third :: new_bitfield_1 (42 , false , bindings :: bitfields :: ItemKind :: ITEM_KIND_TRES ,) , } ; assert ! (unsafe { third . assert (42 , false , bindings :: bitfields :: ItemKind :: ITEM_KIND_TRES) }) ; }
};
}
