macro_rules! deps {
    () => {
        Kind!();
    };
}

macro_rules! Data {
    () => {
        deps!();
        # [doc = " A borrowed object using a slice as backing buffer, or in other words a bytes buffer that knows the kind of object it represents."] # [derive (PartialEq , Eq , Debug , Hash , Ord , PartialOrd , Clone , Copy)] pub struct Data < 'a > { # [doc = " kind of object"] pub kind : Kind , # [doc = " decoded, decompressed data, owned by a backing store."] pub data : & 'a [u8] , }
    };
}

Data!()