macro_rules! deps {
    () => {
        Clone!();
    };
}

macro_rules! OwnedOrStaticAtomicBool {
    () => {
        deps!();
        # [derive (Clone)] pub enum OwnedOrStaticAtomicBool { Owned { flag : Arc < AtomicBool > , # [cfg_attr (not (feature = "parallel") , allow (dead_code))] private : bool , } , Shared (& 'static AtomicBool) , }
    };
}

OwnedOrStaticAtomicBool!();