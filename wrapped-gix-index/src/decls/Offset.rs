macro_rules! Offset {
    () => {
        # [derive (Debug , Clone , Copy)] pub struct Offset { pub from_beginning_of_file : u32 , pub num_entries : u32 , }
    };
}

Offset!()