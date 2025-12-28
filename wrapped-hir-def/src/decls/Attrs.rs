macro_rules! Attrs {
    () => {
        # [doc = " Desugared attributes of an item post `cfg_attr` expansion."] # [derive (Default , Debug , Clone , PartialEq , Eq)] pub struct Attrs (RawAttrs) ;
    };
}

Attrs!()