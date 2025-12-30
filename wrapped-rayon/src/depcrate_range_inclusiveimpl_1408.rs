// Generated macro for impl_1408 (impl)
macro_rules! Depcrate_range_inclusiveimpl_1408 {
() => {
// Module: crate::range_inclusive
// Provides: {"impl_1408"}
// Dependencies: {}
impl IndexedParallelIterator for Iter < char > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { convert_char ! (self . drive (consumer)) } fn len (& self) -> usize { if let Some ((start , end)) = self . bounds () { let start = start as u32 ; let end = end as u32 ; let mut count = end - start ; if start < 0xD800 && 0xE000 <= end { count -= 0x800 } (count + 1) as usize } else { 0 } } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { convert_char ! (self . with_producer (callback)) } }
};
}
