macro_rules! deps {
    () => {
        Format!();
        DebugInfoOffset!();
    };
}

macro_rules! PubStuffHeader {
    () => {
        deps!();
        # [derive (Debug , Clone , PartialEq , Eq)] pub struct PubStuffHeader < T = usize > { format : Format , length : T , version : u16 , unit_offset : DebugInfoOffset < T > , unit_length : T , }
    };
}

PubStuffHeader!();