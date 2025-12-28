macro_rules! Tag {
    () => {
        # [doc = " Single tag in a control group."] # [derive (Copy , Clone , PartialEq , Eq)] # [repr (transparent)] pub (crate) struct Tag (pub (super) u8) ;
    };
}

Tag!();