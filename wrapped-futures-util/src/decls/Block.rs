macro_rules! Block {
    () => {
        # [derive (Debug)] struct Block < Item > { offset : usize , bytes : Item , }
    };
}

Block!()