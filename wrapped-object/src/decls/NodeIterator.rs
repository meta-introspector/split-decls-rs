macro_rules! deps {
    () => {
        Frame!();
    };
}

macro_rules! NodeIterator {
    () => {
        deps!();
        # [derive (Debug)] struct NodeIterator < 'data > { data : & 'data [u8] , offset : usize , stack : Vec < Frame < 'data > > , name_buf : Vec < u8 > , }
    };
}

NodeIterator!()