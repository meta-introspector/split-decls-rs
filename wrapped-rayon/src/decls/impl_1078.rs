macro_rules! deps {
    () => {
        Iter!();
        IndexedParallelIterator!();
        Consumer!();
        ProducerCallback!();
    };
}

macro_rules! impl_1078 {
    () => {
        deps!();
        impl IndexedParallelIterator for Iter < char > { fn drive < C > (self , consumer : C) -> C :: Result where C : Consumer < Self :: Item > , { convert_char ! (self . drive (consumer)) } fn len (& self) -> usize { let start = self . range . start as u32 ; let end = self . range . end as u32 ; if start < end { let mut count = end - start ; if start < 0xD800 && 0xE000 <= end { count -= 0x800 } count as usize } else { 0 } } fn with_producer < CB > (self , callback : CB) -> CB :: Output where CB : ProducerCallback < Self :: Item > , { convert_char ! (self . with_producer (callback)) } }
    };
}

impl_1078!();