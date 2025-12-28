macro_rules! Id {
    () => {
        # [derive (Clone , Debug , Default)] pub (crate) struct Id < 'a > { pub (crate) id : Option < Cow < 'a , str > > , pub (crate) url : Option < Cow < 'a , str > > , }
    };
}

Id!();