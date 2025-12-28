macro_rules! OmitDoc {
    () => {
        # [derive (PartialEq , Clone , Copy , Debug)] pub enum OmitDoc { Lower , Skip , }
    };
}

OmitDoc!()