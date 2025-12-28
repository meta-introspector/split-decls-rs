macro_rules! SizedByDefault {
    () => {
        # [derive (Clone , Copy , PartialEq , Eq)] pub enum SizedByDefault { NotSized , Sized { anchor : Crate } , }
    };
}

SizedByDefault!();