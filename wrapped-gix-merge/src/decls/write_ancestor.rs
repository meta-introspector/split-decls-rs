macro_rules! write_ancestor {
    () => {
        pub fn write_ancestor (input : & InternedInput < & [u8] > , from : u32 , to : usize , out : & mut Vec < u8 >) { if to < from as usize { return ; } if let Some (tokens) = input . before . get (from as usize .. to) { write_tokens (& input . interner , tokens , out) ; } }
    };
}

write_ancestor!()