// Generated macro for impl_126 (impl)
macro_rules! Depcrate_value_iteratorimpl_126 {
() => {
// Module: crate::value_iterator
// Provides: {"impl_126"}
// Dependencies: {}
impl Iterator for ValueIterator < '_ > { type Item = (String , Value) ; fn next (& mut self) -> Option < Self :: Item > { self . range . next () . and_then (| index | { let mut ty = 0 ; let mut name_len = self . name . len () as u32 ; let mut data_len = self . data . len () as u32 ; let result = unsafe { RegEnumValueW (self . key . 0 , index as u32 , self . name . as_mut_ptr () , & mut name_len , core :: ptr :: null () , & mut ty , self . data . as_mut_ptr () , & mut data_len ,) } ; if result != 0 { debug_assert_eq ! (result , ERROR_NO_MORE_ITEMS) ; None } else { let name = String :: from_utf16_lossy (& self . name [0 .. name_len as usize]) ; Some ((name , Value { data : Data :: from_slice (& self . data [0 .. data_len as usize]) , ty : ty . into () , } ,)) } }) } }
};
}
