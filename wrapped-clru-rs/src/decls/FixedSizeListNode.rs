macro_rules! FixedSizeListNode {
    () => {
        # [derive (Debug)] struct FixedSizeListNode < T > { prev : usize , next : usize , data : T , }
    };
}

FixedSizeListNode!()