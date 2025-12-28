macro_rules! repr_for {
    () => {
        # [doc = " Returns the repr attribute to be applied to the resultant ULE or VarULE type"] pub fn repr_for (f : & Fields) -> TokenStream2 { if f . len () == 1 { quote ! (transparent) } else { quote ! (C , packed) } }
    };
}

repr_for!();