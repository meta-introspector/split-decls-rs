macro_rules! VecWriter {
    () => {
        # [derive (Default)] pub (crate) struct VecWriter { inner : Vec < u8 > , }
    };
}

VecWriter!()