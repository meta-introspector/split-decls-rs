macro_rules! deps {
    () => {
        ObjectMap!();
    };
}

macro_rules! ObjectMapEntry {
    () => {
        deps!();
        # [doc = " A symbol in an [`ObjectMap`]."] # [derive (Debug , Default , Clone , Copy , PartialEq , Eq , Hash)] pub struct ObjectMapEntry < 'data > { address : u64 , size : u64 , name : & 'data [u8] , object : usize , }
    };
}

ObjectMapEntry!();