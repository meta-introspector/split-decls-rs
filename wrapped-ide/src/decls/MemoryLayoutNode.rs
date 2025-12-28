macro_rules! MemoryLayoutNode {
    () => {
        pub struct MemoryLayoutNode { pub item_name : String , pub typename : String , pub size : u64 , pub alignment : u64 , pub offset : u64 , pub parent_idx : i64 , pub children_start : i64 , pub children_len : u64 , }
    };
}

MemoryLayoutNode!()