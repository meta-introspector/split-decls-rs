macro_rules! deps {
    () => {
        Bytes!();
        Endian!();
        MachHeader!();
    };
}

macro_rules! LoadCommandIterator {
    () => {
        deps!();
        # [doc = " An iterator for the load commands from a [`MachHeader`]."] # [derive (Debug , Default , Clone , Copy)] pub struct LoadCommandIterator < 'data , E : Endian > { endian : E , data : Bytes < 'data > , ncmds : u32 , }
    };
}

LoadCommandIterator!();