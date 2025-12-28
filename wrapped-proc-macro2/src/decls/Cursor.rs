macro_rules! Cursor {
    () => {
        # [derive (Copy , Clone , Eq , PartialEq)] pub (crate) struct Cursor < 'a > { pub (crate) rest : & 'a str , # [cfg (span_locations)] pub (crate) off : u32 , }
    };
}

Cursor!()