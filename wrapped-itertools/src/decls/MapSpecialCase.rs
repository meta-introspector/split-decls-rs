macro_rules! MapSpecialCase {
    () => {
        # [derive (Clone , Debug)] # [must_use = "iterator adaptors are lazy and do nothing unless consumed"] pub struct MapSpecialCase < I , F > { pub (crate) iter : I , pub (crate) f : F , }
    };
}

MapSpecialCase!();