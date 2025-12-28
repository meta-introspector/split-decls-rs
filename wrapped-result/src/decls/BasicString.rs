macro_rules! BasicString {
    () => {
        # [repr (transparent)] pub struct BasicString (* const u16) ;
    };
}

BasicString!();