macro_rules! deps {
    () => {
        Bytes!();
    };
}

macro_rules! Frame {
    () => {
        deps!();
        # [derive (Debug)] struct Frame < 'data > { data : Bytes < 'data > , children_remaining : u8 , name_buf_len : usize , }
    };
}

Frame!();