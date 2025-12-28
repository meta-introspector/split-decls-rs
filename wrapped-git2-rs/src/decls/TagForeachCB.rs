macro_rules! deps {
    () => {
        Oid!();
    };
}

macro_rules! TagForeachCB {
    () => {
        deps!();
        # [doc = " boxed callback type"] pub (crate) type TagForeachCB < 'a > = Box < dyn FnMut (Oid , & [u8]) -> bool + 'a > ;
    };
}

TagForeachCB!();