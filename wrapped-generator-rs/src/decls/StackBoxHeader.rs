macro_rules! deps {
    () => {
        Stack!();
    };
}

macro_rules! StackBoxHeader {
    () => {
        deps!();
        struct StackBoxHeader { stack : Stack , data_size : usize , need_drop : usize , }
    };
}

StackBoxHeader!()