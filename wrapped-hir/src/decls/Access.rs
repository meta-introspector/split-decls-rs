macro_rules! Access {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq)] pub enum Access { Shared , Exclusive , Owned , }
    };
}

Access!()