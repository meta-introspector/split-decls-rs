macro_rules! ParamValue {
    () => {
        # [doc (hidden)] pub enum ParamValue < T : Type < T > > { Owned (T) , Borrowed (T :: Abi) , }
    };
}

ParamValue!();