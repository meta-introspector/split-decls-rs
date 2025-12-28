macro_rules! RefSpec {
    () => {
        # [doc = " An owned refspec."] # [derive (Eq , Clone , Debug)] pub struct RefSpec { mode : types :: Mode , op : parse :: Operation , src : Option < bstr :: BString > , dst : Option < bstr :: BString > , }
    };
}

RefSpec!()