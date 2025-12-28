macro_rules! AsDisplay {
    () => {
        # [cfg (feature = "serde")] pub (crate) struct AsDisplay < 'a , B > (pub (crate) & 'a B) ;
    };
}

AsDisplay!()